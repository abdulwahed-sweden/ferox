"""Ferox Malfind shim to chain Volatility detection into Ferox reports."""

try:
    from volatility3.plugins.windows import malfind  # type: ignore
except ImportError:  # pragma: no cover
    malfind = object  # type: ignore


class FeroxMalfind(malfind.Malfind):  # type: ignore[misc]
    __doc__ = "Expose malfind results to Ferox bridge"
