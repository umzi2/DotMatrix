from __future__ import annotations

import random
from enum import Enum
from typing import Optional

import numpy as np

class TypeDot(Enum):
    CIRCLE = 0,
    CROSS = 1,
    ELLIPSE = 2,
    LINE = 2,
    INVLINE = 3

def screentone_mask(
    mask_size: tuple[int,int],
    dot_size: int,
    angle: Optional[float] = 0.0,
    dot_type: Optional[TypeDot] = TypeDot.CIRCLE
)