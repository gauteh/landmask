# -*- coding: utf-8 -*-
from landmask import Landmask
import os.path
import numpy as np
import shapely
import shapely.vectorized
from shapely import wkb

shapes = os.path.join(os.path.dirname(__file__), '../..') + os.path.sep
coarse = os.path.join (shapes, 'coarse.wkb')
high = os.path.join (shapes, 'high.wkb')

def test_landmask_load_coarse(benchmark):
  benchmark (Landmask, coarse)

def test_shapely_load_coarse(benchmark):
  def l():
    with open(coarse, 'rb') as fd:
      land = wkb.load (fd)
    prep = shapely.prepared.prep(land)
  benchmark(l)

def test_landmask_load_high(benchmark):
  benchmark (Landmask, high)

def test_shapely_load_high(benchmark):
  def l():
    with open(high, 'rb') as fd:
      land = wkb.load (fd)
    prep = shapely.prepared.prep(land)
  benchmark(l)

def test_coarse_many_points(benchmark):
  xx, yy = np.mgrid[-180:180,-90:90] # 64800 points
  xx = xx.astype('float64')
  yy = yy.astype('float64')

  l = Landmask(coarse)
  print ("testing %d points.." % len(xx.ravel()))

  contains = benchmark(l.contains_many, xx.ravel(), yy.ravel())
  # contains = l.contains_many (xx.ravel(), yy.ravel())
  assert np.any(contains)

def test_coarse_many_points_shapely(benchmark):
  xx, yy = np.mgrid[-180:180,-90:90] # 64800 points
  xx = xx.astype('float64')
  yy = yy.astype('float64')


  with open(coarse, 'rb') as fd:
    land = wkb.load (fd)
  prep = shapely.prepared.prep(land)

  contains = benchmark(shapely.vectorized.contains, prep, xx, yy)
  assert np.any(contains)

def test_high_many_points(benchmark):
  xx, yy = np.mgrid[-180:180,-90:90] # 64800 points
  xx = xx.astype('float64')
  yy = yy.astype('float64')

  l = Landmask(high)
  print ("testing %d points.." % len(xx.ravel()))

  contains = benchmark(l.contains_many, xx.ravel(), yy.ravel())
  # contains = l.contains_many (xx.ravel(), yy.ravel())
  assert np.any(contains)

def test_high_many_points_shapely(benchmark):
  xx, yy = np.mgrid[-180:180,-90:90] # 64800 points
  xx = xx.astype('float64')
  yy = yy.astype('float64')


  with open(high, 'rb') as fd:
    land = wkb.load (fd)
  prep = shapely.prepared.prep(land)

  contains = benchmark(shapely.vectorized.contains, prep, xx, yy)
  assert np.any(contains)
