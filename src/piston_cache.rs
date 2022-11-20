//! This functions as a basic cacher for `Piston2D` images
//!
//! ## Usage
//! ```rust
//!
//! use burntnail_utils::piston_cache::Cacher;
//! let mut cacher = Cacher::new(&mut get_anything_for_docs(), Some("assets"));
//!
//! //then, we can either insert a bunch of textures on start
//! cacher.insert("sprite.png")?;
//! cacher.insert("bg.png")?;
//!
//! //or, just grab them as and when we need them
//! cacher.get("highly-specific-level-thingie.png");
//!
//! ```

use crate::time_based_structs::scoped_timers::ScopedTimer;
use find_folder::Search::ParentsThenKids;
use piston_window::{
    Filter, Flip, G2dTexture, G2dTextureContext, PistonWindow, Texture, TextureSettings,
};
use std::{collections::HashMap, path::PathBuf, result::Result as SResult};

use crate::{
    error_ext::ToErr,
    error_types::{Error, Result},
};

///Struct to hold a cache of [`G2dTexture`]s
pub struct Cacher {
    ///Base path for the assets
    base_path: PathBuf,
    ///HashMap of paths to textures
    assets: HashMap<String, G2dTexture>,
    ///Context for textures from window
    tc: G2dTextureContext,
}

impl Cacher {
    ///Function to create a new empty cache.
    ///
    /// # Errors
    /// Can fail if it can't find the assets folder
    fn base_new(win: &mut PistonWindow, path: Option<&str>) -> SResult<Self, find_folder::Error> {
        let path = ParentsThenKids(2, 2).for_folder(path.unwrap_or("assets"))?;

        Ok(Self {
            base_path: path,
            assets: HashMap::new(),
            tc: win.create_texture_context(),
        })
    }

    ///Base function for getting something
    ///
    ///Takes a relative path, returns either `Err(String)` from insertion, or an `Ok(G2DTexture)` with the result from the hashmap if insertion had no errors
    fn base_get(&mut self, p: &str) -> SResult<&G2dTexture, String> {
        self.base_insert(p)
            .map(|_| {
                self.assets
                    .get(p)
                    .ok_or_else(|| "Asset missing in internal storage".into())
            })
            .and_then(std::convert::identity) //Taken from the unstable code, issue: 70142, nice code: `.flatten()`
    }

    ///Base function for inserting something.
    ///
    ///Takes a relative path, returns `Ok` if all worked or element already existed, else returns `Err(String)` if failure
    fn base_insert(&mut self, p: &str) -> SResult<(), String> {
        if self.assets.contains_key(p) {
            return Ok(());
        }

        #[cfg(feature = "tracing")]
        tracing::trace!("Inserting {p}");
        #[cfg(not(feature = "tracing"))]
        println!("Inserting {p}");

        let _st = ScopedTimer::new(format!("Geting {p}"));

        let path = self.base_path.join(p);
        let ts = TextureSettings::new().filter(Filter::Nearest);

        match Texture::from_path(&mut self.tc, path, Flip::None, &ts) {
            Ok(tex) => {
                self.assets.insert(p.to_string(), tex);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

impl Cacher {
    ///Function to create a new empty cache.
    ///
    /// # Errors
    /// Can fail if it can't find the assets folder
    pub fn new(win: &mut PistonWindow, path: Option<&str>) -> Result<Self> {
        Self::base_new(win, path).ae()
    }

    ///Gets a [`G2dTexture`] from the cache. Returns [`None`] if there is no asset with that path.
    ///
    /// # Errors
    /// - Unable to find the texture using [`Texture::from_path`]
    pub fn get(&mut self, p: &str) -> Result<&G2dTexture> {
        match self.base_get(p) {
            Ok(tex) => Ok(tex),
            Err(e) => Err(Error::msg(format!("Texture Get Error: {e}"))),
        }
    }

    ///Inserts a new asset into the cache from the path given - should just be like `'icon.png'`, as all files should be in the `'assets/'` folder
    ///
    /// # Errors
    /// - Unable to find the texture using [`Texture::from_path`]
    pub fn insert(&mut self, p: &str) -> Result<()> {
        self.base_insert(p)
            .map_err(|s| Error::msg(format!("Texture Insert Error: {s}")))
    }
}
