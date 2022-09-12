use crate::{time_based_structs::scoped_timers::ScopedTimer};
use find_folder::Search::ParentsThenKids;
use piston_window::{
    Filter, Flip, G2dTexture, G2dTextureContext, PistonWindow, Texture, TextureSettings,
};
use std::{collections::HashMap, path::PathBuf, result::Result as SResult};

#[cfg(feature = "anyhow")]
use crate::error_ext::{ToAnyhowErr};
#[cfg(feature = "anyhow")]
use anyhow::{anyhow, Result};

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
    fn base_new(win: &mut PistonWindow) -> SResult<Self, find_folder::Error> {
        let path = ParentsThenKids(2, 2).for_folder("assets")?;

        Ok(Self {
            base_path: path,
            assets: HashMap::new(),
            tc: win.create_texture_context(),
        })
    }

    fn base_get(&mut self, p: &str) -> SResult<Option<&G2dTexture>, String> {
        self.base_insert(p).map(|_| self.assets.get(p))
    }

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

#[cfg(feature = "anyhow")]
impl Cacher {
    ///Function to create a new empty cache.
    ///
    /// # Errors
    /// Can fail if it can't find the assets folder
    pub fn new(win: &mut PistonWindow) -> Result<Self> {
        Cacher::base_new(win).ae()
    }

    ///Gets a [`G2dTexture`] from the cache. Returns [`None`] if there is no asset with that path.
    ///
    /// # Errors
    /// - Unable to find the texture using [`Texture::from_path`]
    pub fn get(&mut self, p: &str) -> Result<&G2dTexture> {
        match self.base_get(p) {
            Ok(opt) => match opt {
                Some(ok) => Ok(ok),
                None => Err(anyhow!("Finding asset that exists")),
            },
            Err(e) => Err(anyhow!("{e}")),
        }
    }

    ///Inserts a new asset into the cache from the path given - should just be like `'icon.png'`, as all files should be in the `'assets/'` folder
    ///
    /// # Errors
    /// - Unable to find the texture using [`Texture::from_path`]
    pub fn insert(&mut self, p: &str) -> Result<()> {
        self.base_insert(p).map_err(|s| anyhow!("{s}"))
    }
}

#[cfg(not(feature = "anyhow"))]
impl Cacher {
    ///Function to create a new empty cache.
    ///
    /// # Errors
    /// Can fail if it can't find the assets folder
    pub fn new(win: &mut PistonWindow) -> SResult<Self, find_folder::Error> {
        Cacher::base_new(win)
    }

    ///Gets a [`G2dTexture`] from the cache. Returns [`None`] if there is no asset with that path.
    ///
    /// # Errors
    /// - Unable to find the texture using [`Texture::from_path`]
    pub fn get(&mut self, p: &str) -> SResult<&G2dTexture, String> {
        match self.base_get(p) {
            Ok(opt) => match opt {
                Some(ok) => Ok(ok),
                None => Err("Unable to find asset".into()),
            },
            Err(e) => Err(e),
        }
    }

    ///Inserts a new asset into the cache from the path given - should just be like `'icon.png'`, as all files should be in the `'assets/'` folder
    ///
    /// # Errors
    /// - Unable to find the texture using [`Texture::from_path`]
    pub fn insert(&mut self, p: &str) -> SResult<(), String> {
        self.base_insert(p)
    }
}
