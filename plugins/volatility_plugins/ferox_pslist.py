"""Ferox custom pslist wrapper for Volatility 3."""

try:
    from volatility3.framework import interfaces  # type: ignore
    from volatility3.framework.renderers import format_hints  # type: ignore
    from volatility3.plugins.windows import pslist  # type: ignore
except ImportError:  # pragma: no cover - Volatility optional runtime dependency
    interfaces = object  # type: ignore
    format_hints = object  # type: ignore
    pslist = object  # type: ignore


class FeroxPsList(pslist.PsList):  # type: ignore[misc]
    __doc__ = "Rust-powered Ferox pslist facade"

    @classmethod
    def get_requirements(cls):  # type: ignore[override]
        requirements = list(super().get_requirements())  # type: ignore[attr-defined]
        return requirements

    def _generator(self):  # type: ignore[override]
        for task in super()._generator():  # type: ignore[attr-defined]
            yield task
