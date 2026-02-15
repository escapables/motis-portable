import { expect, test } from '@playwright/test';

const setupInitialRoute = async (page: import('@playwright/test').Page) => {
	await page.route('**/api/v1/map/initial', async (route) => {
		await route.fulfill({
			status: 200,
			contentType: 'application/json',
			body: JSON.stringify({
				lat: 59.3293,
				lon: 18.0686,
				zoom: 12,
				serverConfig: {
					hasElevation: false,
					hasRoutedTransfers: true,
					hasStreetRouting: true,
					maxPrePostTransitTimeLimit: 7200,
					maxDirectTimeLimit: 7200
				}
			})
		});
	});
};

test('@smoke app shell renders with map pin controls', async ({ page }) => {
	await setupInitialRoute(page);

	await page.goto('/');
	await expect(page.getByRole('tab', { name: /Connections/i })).toBeVisible();
	await expect(page.getByRole('tab', { name: /Departures/i })).toBeVisible();
	await expect(page.getByRole('tab', { name: /Isochrones/i })).toBeVisible();
	await expect(page.getByTestId('from-map-pin')).toBeVisible();
	await expect(page.getByTestId('to-map-pin')).toBeVisible();
});

test('@regression preserves departures stopId across repeated tab switches', async ({ page }) => {
	await setupInitialRoute(page);

	await page.route('**/api/v5/stoptimes**', async (route) => {
		await route.fulfill({
			status: 200,
			contentType: 'application/json',
			body: JSON.stringify({
				place: {
					name: 'Test Stop',
					lat: 59.3293,
					lon: 18.0686
				},
				stopTimes: [],
				previousPageCursor: '',
				nextPageCursor: ''
			})
		});
	});

	await page.route('**/api/v1/one-to-all**', async (route) => {
		await route.fulfill({
			status: 200,
			contentType: 'application/json',
			body: JSON.stringify({
				all: [
					{
						place: { lat: 59.3293, lon: 18.0686, name: 'Test Stop' },
						duration: 1
					}
				]
			})
		});
	});

	await page.goto(
		'/?fromPlace=stop-a&fromName=Test%20Stop&stopId=stop-a&time=2026-02-14T10:00:00.000Z'
	);

	const departuresTab = page.getByRole('tab', { name: /Departures/i });
	const connectionsTab = page.getByRole('tab', { name: /Connections/i });
	const isochronesTab = page.getByRole('tab', { name: /Isochrones/i });

	await expect(departuresTab).toHaveAttribute('data-state', 'active');
	await expect.poll(() => new URL(page.url()).searchParams.get('stopId')).toBe('stop-a');

	const switchSequence = [
		isochronesTab,
		connectionsTab,
		departuresTab,
		isochronesTab,
		departuresTab,
		connectionsTab,
		departuresTab
	];

	for (const tab of switchSequence) {
		await tab.click();
	}

	await departuresTab.click();
	await expect(departuresTab).toHaveAttribute('data-state', 'active');
	await expect.poll(() => new URL(page.url()).searchParams.get('stopId')).toBe('stop-a');
});

test('@regression cancels pending isochrones request when leaving tab quickly', async ({
	page
}) => {
	await setupInitialRoute(page);

	let oneToAllRequestCount = 0;
	await page.route('**/api/v1/one-to-all**', async (route) => {
		oneToAllRequestCount += 1;
		await route.fulfill({
			status: 200,
			contentType: 'application/json',
			body: JSON.stringify({ all: [] })
		});
	});

	await page.goto('/?fromPlace=stop-a&fromName=Test%20Stop');

	await page.evaluate(() => {
		const tabs = Array.from(document.querySelectorAll('[role="tab"]'));
		const isochrones = tabs.find((tab) => /Isochrones/i.test(tab.textContent ?? ''));
		const connections = tabs.find((tab) => /Connections/i.test(tab.textContent ?? ''));
		isochrones?.dispatchEvent(new MouseEvent('click', { bubbles: true }));
		connections?.dispatchEvent(new MouseEvent('click', { bubbles: true }));
	});

	await page.waitForTimeout(200);
	expect(oneToAllRequestCount).toBe(0);
	await expect(page.getByRole('tab', { name: /Connections/i })).toHaveAttribute(
		'data-state',
		'active'
	);
});
