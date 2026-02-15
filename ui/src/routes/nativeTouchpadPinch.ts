import maplibregl from 'maplibre-gl';

export const GDK_TOUCHPAD_GESTURE_PHASE_BEGIN = 0;
export const GDK_TOUCHPAD_GESTURE_PHASE_UPDATE = 1;
export const GDK_TOUCHPAD_GESTURE_PHASE_END = 2;
export const GDK_TOUCHPAD_GESTURE_PHASE_CANCEL = 3;

export type NativeTouchpadPinchDetail = {
	x: number;
	y: number;
	scale: number;
	phase: number;
};

export const applyNativeTouchpadPinch = (
	detail: NativeTouchpadPinchDetail | undefined,
	map: maplibregl.Map | undefined,
	lastScale: number | undefined,
	elementFromPoint: (x: number, y: number) => Element | null
): number | undefined => {
	if (!detail) {
		return lastScale;
	}
	if (!Number.isFinite(detail.scale) || detail.scale <= 0) {
		return lastScale;
	}
	if (detail.phase === GDK_TOUCHPAD_GESTURE_PHASE_BEGIN) {
		return detail.scale;
	}
	if (
		detail.phase === GDK_TOUCHPAD_GESTURE_PHASE_END ||
		detail.phase === GDK_TOUCHPAD_GESTURE_PHASE_CANCEL
	) {
		return undefined;
	}
	if (detail.phase !== GDK_TOUCHPAD_GESTURE_PHASE_UPDATE || !map) {
		return lastScale;
	}
	if (!Number.isFinite(detail.x) || !Number.isFinite(detail.y)) {
		return lastScale;
	}

	const target = elementFromPoint(detail.x, detail.y);
	if (!(target instanceof Element) || !target.closest('.motis-map')) {
		return lastScale;
	}

	const previousScale = lastScale ?? detail.scale;
	const nextScale = detail.scale;
	if (!Number.isFinite(previousScale) || previousScale <= 0) {
		return nextScale;
	}

	const scaleRatio = detail.scale / previousScale;
	if (!Number.isFinite(scaleRatio) || Math.abs(scaleRatio - 1) < 0.001) {
		return nextScale;
	}

	const zoomDelta = Math.log2(scaleRatio);
	if (!Number.isFinite(zoomDelta) || Math.abs(zoomDelta) < 0.001) {
		return nextScale;
	}

	const around = map.unproject([detail.x, detail.y]);
	const nextZoom = Math.max(
		map.getMinZoom(),
		Math.min(map.getMaxZoom(), map.getZoom() + zoomDelta)
	);
	map.zoomTo(nextZoom, { around, duration: 0 });
	return nextScale;
};
