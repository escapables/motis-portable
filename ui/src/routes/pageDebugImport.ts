import type { PlanResponse } from '@motis-project/motis-client';
import { parseDebugPlanResponse } from '$lib/debugPlanImport';

const isEditablePasteTarget = (target: EventTarget | null): boolean => {
	return Boolean(
		target instanceof HTMLElement &&
			target.closest('input, textarea, [contenteditable], [role="textbox"]')
	);
};

const toClipboardText = (event: ClipboardEvent): string => {
	return event.clipboardData?.getData('text/plain') ?? event.clipboardData?.getData('text') ?? '';
};

export const registerDebugPlanClipboardImport = (
	windowObj: Window,
	onImport: (response: Promise<PlanResponse>) => void
): (() => void) => {
	const handlePaste = (event: ClipboardEvent) => {
		if (event.defaultPrevented || isEditablePasteTarget(event.target)) {
			return;
		}
		const parsed = parseDebugPlanResponse(toClipboardText(event));
		if (!parsed) {
			return;
		}
		onImport(Promise.resolve(parsed));
	};
	windowObj.addEventListener('paste', handlePaste);
	return () => {
		windowObj.removeEventListener('paste', handlePaste);
	};
};
