import { expect, test } from '@playwright/test';

test('@smoke app shell renders with map pin controls', async ({ page }) => {
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

	await page.goto('/');
	await expect(page.getByRole('tab', { name: /Connections/i })).toBeVisible();
	await expect(page.getByRole('tab', { name: /Departures/i })).toBeVisible();
	await expect(page.getByRole('tab', { name: /Isochrones/i })).toBeVisible();
	await expect(page.getByTestId('from-map-pin')).toBeVisible();
	await expect(page.getByTestId('to-map-pin')).toBeVisible();
});
