import { test, expect } from '@playwright/test';
import { injectTauriMock, mockCommand } from './helpers/tauri-mock';
import { traceViewNoErrors, traceViewWithErrors } from './fixtures/trace';

test.beforeEach(async ({ page }) => {
  await injectTauriMock(page);
  await page.goto('/');
});

test('shows placeholder when no trace is loaded', async ({ page }) => {
  await expect(page.getByText('Load a trace to begin')).toBeVisible();
});

test('shows all metadata fields after loading a trace', async ({ page }) => {
  await mockCommand(page, 'load_trace', traceViewNoErrors);
  await page.getByRole('button', { name: 'Load Trace' }).click();

  await expect(page.getByText('frontend-service')).toBeVisible();
  await expect(page.getByText('/api/v1/checkout')).toBeVisible();
  await expect(page.getByText('abc123def456abc123def456abc123de')).toBeVisible();
  await expect(page.getByText('452.00ms')).toBeVisible();
  await expect(page.getByText('3', { exact: true })).toBeVisible();
  await expect(page.getByText('12', { exact: true })).toBeVisible();
});

test('error count shows in secondary color when zero', async ({ page }) => {
  await mockCommand(page, 'load_trace', traceViewNoErrors);
  await page.getByRole('button', { name: 'Load Trace' }).click();

  const errorValue = page.locator('.error-count');
  await expect(errorValue).toBeVisible();
  await expect(errorValue).toHaveText('0');
  await expect(errorValue).not.toHaveClass(/error-nonzero/);
});

test('error count shows in error color when non-zero', async ({ page }) => {
  await mockCommand(page, 'load_trace', traceViewWithErrors);
  await page.getByRole('button', { name: 'Load Trace' }).click();

  const errorValue = page.locator('.error-count');
  await expect(errorValue).toHaveText('4');
  await expect(errorValue).toHaveClass(/error-nonzero/);
});

test('placeholder disappears after trace is loaded', async ({ page }) => {
  await expect(page.getByText('Load a trace to begin')).toBeVisible();

  await mockCommand(page, 'load_trace', traceViewNoErrors);
  await page.getByRole('button', { name: 'Load Trace' }).click();

  await expect(page.getByText('Load a trace to begin')).not.toBeVisible();
});
