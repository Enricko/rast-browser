# rast-browser

A web-viewer app for the RAST Package Center, built with the RAST package
template. It opens public HTTPS pages through RAST's sandboxed page proxy
(`/api/browser/page`), with an address bar, back/forward, reload and home.
Text that is not an address is searched on Brave Search and the home page is
Google. Google's own results page refuses server-side fetches from datacenter
IPs (it answers with a JavaScript-only "redirecting" page), so queries use
Brave, which renders as plain HTML through the proxy.

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
