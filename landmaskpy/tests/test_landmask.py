# -*- coding: utf-8 -*-
from landmask import Landmask
import opendrift_landmask_data as old
import numpy as np

def test_load_coarse(benchmark):
  benchmark (Landmask, old.GSHHS['c'])

def test_many_points(benchmark):
  xx, yy = np.mgrid[-180:180,-90:90] # 64800 points
  l = Landmask(old.GSHHS['c'])
  xx = xx.astype('float64')
  yy = yy.astype('float64')

  print ("testing %d points.." % len(xx.ravel()))

  contains = benchmark(l.contains_many, xx.ravel(), yy.ravel())
  # contains = l.contains_many (xx.ravel(), yy.ravel())

  assert np.any(contains)

