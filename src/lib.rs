#![feature(test)]
extern crate test;

use std::io;
use std::fs::File;
use wkb;
use geo::*;
use geo::algorithm::bounding_rect::BoundingRect;
use geo::algorithm::euclidean_distance::EuclideanDistance;
use geo::algorithm::contains::*;
pub use rstar::{RTree, RTreeParams, RStarInsertionStrategy, RTreeObject,
                AABB, PointDistance};
use rayon::prelude::*;

mod tests;

pub struct LargeNodeParameters;

impl RTreeParams for LargeNodeParameters
{
    const MIN_SIZE: usize = 100;
    const MAX_SIZE: usize = 200;
    const REINSERTION_COUNT: usize = 50;
    type DefaultInsertionStrategy = RStarInsertionStrategy;
}

// Optional but helpful: Define a type alias for the new r-tree
pub type LargeNodeRTree<T> = RTree<T, LargeNodeParameters>;

pub struct PolyWrapper(geo::Polygon<f64>);

impl RTreeObject for PolyWrapper
{
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        let r = self.0.bounding_rect().unwrap();
        AABB::from_corners([r.min.x, r.min.y], [r.max.x, r.max.y])
    }
}

impl PointDistance for PolyWrapper
{
    fn distance_2(&self, point: &[f64; 2]) -> f64 {
        let p = geo::Point::new(point[0], point[1]);
        p.euclidean_distance(&self.0)
    }

    fn contains_point(&self, point: &[f64; 2]) -> bool {
        let p = geo::Point::new(point[0], point[1]);
        self.0.contains(&p)
    }
}

pub fn contains<T>(tree: &LargeNodeRTree<T>, x: f64, y: f64) -> bool
where T: RTreeObject<Envelope = AABB<[f64; 2]>> + PointDistance
{
    let pnt: [f64; 2] = [x, y];
    let poly = tree.locate_at_point(&pnt);

    match poly {
        Some(_) => true,
        _       => false
    }
}

pub fn contains_many(tree: &LargeNodeRTree<PolyWrapper>, x: &[f64], y: &[f64])
    -> Vec<bool>
{
    // println! ("contains many: {}", x.len());
    x.par_iter()
     .zip(y)
     .map (|(xx, yy)| contains(&tree, *xx, *yy))
     .collect()
}

pub fn make_rtree_wkb(file: &str) -> io::Result<LargeNodeRTree<PolyWrapper>>
{
    println! ("opening wkb: {}", file);

    let mut f = File::open(file)?;
    let geom = wkb::wkb_to_geom(&mut f).unwrap();

    println! ("creating rtree");
    let tree = if let Geometry::MultiPolygon(mp) = geom {
        RTree::bulk_load_with_params(mp.into_iter().map(|p| PolyWrapper(p)).collect())
    } else {
        panic! ("Could not build RTree.")
    };

    println! ("rtree created, size: {}", tree.size());

    Ok(tree)
}

