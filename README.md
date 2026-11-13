# rast-browser

A web-viewer app for the RAST Package Center, built with the RAST package
template. It opens public HTTPS pages through RAST's sandboxed page proxy
(`/api/browser/page`), with an address bar, back/forward, reload and home.
Opening the app shows a local start page (it needs no network), with a search
box and quick links. Text that is not an address is searched on Google; when
Google answers with its JavaScript-only anti-bot page (server fetches from
datacenter IPs), RAST retries the query on Brave Search, whose HTML results
render through the proxy. Fetch errors are shown as readable pages, never as
raw JSON.

Limitations: pages are fetched server-side and scripts run in a sandboxed
frame, so sites that need cookies, same-origin storage, logins or heavy
single-page JavaScript may not work. Only HTTPS on port 443 is fetched.

## Install

Package Center → **Install from GitHub** → paste
<https://github.com/Enricko/rast-browser/releases/latest>; the app appears
under **Apps** with an **Open** button.

Requires RAST 0.11.11 or newer (the page proxy).

## Publish

Push a tag like `v1.0.0-1`; the release workflow builds both targets and
publishes `rast-package.json` and the archives.
