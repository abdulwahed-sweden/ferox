"""Ferox netscan adapter."""

try:
    from volatility3.plugins.windows import netscan  # type: ignore
except ImportError:  # pragma: no cover
    netscan = object  # type: ignore


class FeroxNetScan(netscan.NetScan):  # type: ignore[misc]
    __doc__ = "Reuse Volatility NetScan stream"
