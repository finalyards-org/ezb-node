# Troubleshoot

## Do *not* keep an `sdkconfig` in workspace root

It's tempting to leave one there, if you have made local copies or something. But it **does seem to confuse builds**.

Instead of following `sdkconfig.defaults`, things in the `sdkconfig` are taken as the truth.

This is uncharted. Just be aware.

