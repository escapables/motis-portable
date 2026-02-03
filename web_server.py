#!/usr/bin/env python3
"""
Simple HTTP server for MOTIS GUI.
Serves the static HTML and proxies API calls to the C++ backend.
"""

import http.server
import socketserver
import json
import subprocess
import os
import sys
from urllib.parse import urlparse, parse_qs
from pathlib import Path

PORT = 8080

# Get the directory where this script is located
SCRIPT_DIR = Path(__file__).parent.resolve()

# Default paths relative to script location
DEFAULT_DATA_PATH = SCRIPT_DIR / 'data'
DEFAULT_BACKEND = SCRIPT_DIR / 'motis-ipc'

# Allow override via environment
DATA_PATH = Path(os.environ.get('MOTIS_DATA', DEFAULT_DATA_PATH))
BACKEND_EXE = Path(os.environ.get('MOTIS_BACKEND', DEFAULT_BACKEND))

print(f"Script directory: {SCRIPT_DIR}")
print(f"Data path: {DATA_PATH} (exists: {DATA_PATH.exists()})")
print(f"Backend: {BACKEND_EXE} (exists: {BACKEND_EXE.exists()})")

# Check prerequisites
if not DATA_PATH.exists():
    print(f"ERROR: Data path does not exist: {DATA_PATH}")
    print("Create a symlink or set MOTIS_DATA environment variable")
    print(f"Example: ln -s /path/to/motis/build/data {DATA_PATH}")
    sys.exit(1)

if not BACKEND_EXE.exists():
    print(f"ERROR: Backend executable not found: {BACKEND_EXE}")
    print("Copy motis-native-example to motis-ipc or set MOTIS_BACKEND")
    print(f"Example: cp /path/to/motis/build/native/motis-native-example {BACKEND_EXE}")
    sys.exit(1)

class MotisHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        # Serve files from gui/src directory
        web_root = SCRIPT_DIR / 'gui' / 'src'
        if not web_root.exists():
            web_root = SCRIPT_DIR / 'src'
        super().__init__(*args, directory=str(web_root), **kwargs)
    
    def log_message(self, format, *args):
        # Suppress default logging
        pass
    
    def do_GET(self):
        parsed = urlparse(self.path)
        
        if parsed.path == '/api/geocode':
            query = parse_qs(parsed.query).get('q', [''])[0]
            result = self.call_backend({'cmd': 'geocode', 'query': query})
            self.send_json(result)
        elif parsed.path == '/api/plan_route':
            params = parse_qs(parsed.query)
            try:
                result = self.call_backend({
                    'cmd': 'plan_route',
                    'from_lat': float(params.get('from_lat', [0])[0]),
                    'from_lon': float(params.get('from_lon', [0])[0]),
                    'to_lat': float(params.get('to_lat', [0])[0]),
                    'to_lon': float(params.get('to_lon', [0])[0])
                })
                self.send_json(result)
            except (ValueError, KeyError) as e:
                self.send_json({'status': 'error', 'message': f'Invalid parameters: {e}'})
        else:
            super().do_GET()
    
    def call_backend(self, request):
        try:
            proc = subprocess.run(
                [str(BACKEND_EXE), str(DATA_PATH)],
                input=json.dumps(request) + '\n',
                capture_output=True,
                text=True,
                timeout=30
            )
            if proc.returncode != 0:
                return {'status': 'error', 'message': f'Backend error: {proc.stderr}'}
            try:
                return json.loads(proc.stdout)
            except json.JSONDecodeError as e:
                return {'status': 'error', 'message': f'Invalid JSON: {e}', 'raw': proc.stdout[:200]}
        except subprocess.TimeoutExpired:
            return {'status': 'error', 'message': 'Backend timeout'}
        except Exception as e:
            return {'status': 'error', 'message': str(e)}
    
    def send_json(self, data):
        self.send_response(200)
        self.send_header('Content-type', 'application/json')
        self.send_header('Access-Control-Allow-Origin', '*')
        self.end_headers()
        self.wfile.write(json.dumps(data).encode())

if __name__ == '__main__':
    print(f"\nStarting MOTIS web server on http://localhost:{PORT}")
    print(f"Press Ctrl+C to stop\n")
    
    with socketserver.TCPServer(("", PORT), MotisHandler) as httpd:
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\nShutting down...")
