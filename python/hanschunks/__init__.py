import os
from platform import system

# On Windows there is no equivalent way of setting RPATH
# This adds the current directory to PATH so that the graalvm libs will be found
if system() == "Windows":
    libpath = os.path.dirname(__file__)
    os.environ["PATH"] = libpath + os.pathsep + os.environ["PATH"]

from .hanschunks import *  # noqa: F403

__doc__ = hanschunks.__doc__  # type: ignore # noqa: F405
__all__ = hanschunks.__all__  # type: ignore # noqa: F405
