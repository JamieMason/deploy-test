use {
  crate::{
    dependency::Dependency, group_selector::GroupSelector, instance::Instance, package_json::PackageJson, packages::Packages,
    specifier::Specifier,
  },
  log::warn,
  serde::Deserialize,
  std::{cell::RefCell, collections::BTreeMap, rc::Rc, vec},
};

/// What behaviour has this group been configured to exhibit?
#[derive(Clone, Debug)]
pub enum VersionGroupVariant {
  Banned,
  HighestSemver,
  Ignored,
  LowestSemver,
  Pinned,
  SameRange,
  SnappedTo,
}

#[derive(Debug)]
pub struct VersionGroup {
  /// Group instances of each dependency together for comparison.
  pub dependencies: RefCell<BTreeMap<String, Dependency>>,
  /// The version to pin all instances to when variant is `Pinned`
  pub pin_version: Option<Specifier>,
  /// Data to determine which instances should be added to this group
  pub selector: GroupSelector,
  /// package.json files whose names match the `snapTo` config when variant is
  /// `SnappedTo`
  pub snap_to: Option<Vec<Rc<RefCell<PackageJson>>>>,
  /// What behaviour has this group been configured to exhibit?
  pub variant: VersionGroupVariant,
}

impl VersionGroup {
  /// Create a default/catch-all group which would apply to any instance
  pub fn get_catch_all() -> VersionGroup {
    VersionGroup {
      dependencies: RefCell::new(BTreeMap::new()),
      pin_version: None,
      selector: GroupSelector::new(
        /* include_dependencies: */ vec![],
        /* include_dependency_types: */ vec![],
        /* label: */ "Default Version Group".to_string(),
        /* include_packages: */ vec![],
        /* include_specifier_types: */ vec![],
      ),
      snap_to: None,
      variant: VersionGroupVariant::HighestSemver,
    }
  }

  pub fn add_instance(&self, instance: Rc<Instance>) {
    let mut dependencies = self.dependencies.borrow_mut();
    let dependency = dependencies.entry(instance.name.clone()).or_insert_with(|| {
      Dependency::new(
        /* name: */ instance.name.clone(),
        /* variant: */ self.variant.clone(),
        /* pin_version: */ self.pin_version.clone(),
        /* snap_to: */ self.snap_to.clone(),
      )
    });
    dependency.add_instance(Rc::clone(&instance));
    std::mem::drop(dependencies);
  }

  /// Create a single version group from a config item from the rcfile.
  pub fn from_config(group: &AnyVersionGroup, packages: &Packages) -> VersionGroup {
    let selector = GroupSelector::new(
      /* include_dependencies: */
      with_resolved_keywords(&group.dependencies, packages),
      /* include_dependency_types: */ group.dependency_types.clone(),
      /* label: */ group.label.clone(),
      /* include_packages: */ group.packages.clone(),
      /* include_specifier_types: */ group.specifier_types.clone(),
    );

    if let Some(true) = group.is_banned {
      return VersionGroup {
        dependencies: RefCell::new(BTreeMap::new()),
        pin_version: None,
        selector,
        snap_to: None,
        variant: VersionGroupVariant::Banned,
      };
    }
    if let Some(true) = group.is_ignored {
      return VersionGroup {
        dependencies: RefCell::new(BTreeMap::new()),
        pin_version: None,
        selector,
        snap_to: None,
        variant: VersionGroupVariant::Ignored,
      };
    }
    if let Some(pin_version) = &group.pin_version {
      return VersionGroup {
        dependencies: RefCell::new(BTreeMap::new()),
        pin_version: Some(Specifier::new(pin_version)),
        selector,
        snap_to: None,
        variant: VersionGroupVariant::Pinned,
      };
    }
    if let Some(policy) = &group.policy {
      if policy == "sameRange" {
        return VersionGroup {
          dependencies: RefCell::new(BTreeMap::new()),
          pin_version: None,
          selector,
          snap_to: None,
          variant: VersionGroupVariant::SameRange,
        };
      } else {
        // @FIXME: show user friendly error message and exit with error code
        panic!("Unrecognised version group policy: {}", policy);
      }
    }
    if let Some(snap_to) = &group.snap_to {
      return VersionGroup {
        dependencies: RefCell::new(BTreeMap::new()),
        pin_version: None,
        selector,
        snap_to: Some(
          snap_to
            .iter()
            .flat_map(|name| {
              packages.get_by_name(name).or_else(|| {
                // @FIXME: show user friendly error message and exit with error code
                warn!("Invalid Snapped To Version Group: No package.json file found with a name property of '{name}'");
                None
              })
            })
            .collect(),
        ),
        variant: VersionGroupVariant::SnappedTo,
      };
    }
    if let Some(prefer_version) = &group.prefer_version {
      return VersionGroup {
        dependencies: RefCell::new(BTreeMap::new()),
        pin_version: None,
        selector,
        snap_to: None,
        variant: if prefer_version == "lowestSemver" {
          VersionGroupVariant::LowestSemver
        } else {
          VersionGroupVariant::HighestSemver
        },
      };
    }
    VersionGroup {
      dependencies: RefCell::new(BTreeMap::new()),
      pin_version: None,
      selector,
      snap_to: None,
      variant: VersionGroupVariant::HighestSemver,
    }
  }
}

struct SnapToMismatches {
  pub instance_ids: Vec<String>,
  pub actual_specifier: Specifier,
  pub expected_specifier: Specifier,
  pub snap_to_instance_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnyVersionGroup {
  #[serde(default)]
  pub dependencies: Vec<String>,
  #[serde(default)]
  pub dependency_types: Vec<String>,
  #[serde(default)]
  pub label: String,
  #[serde(default)]
  pub packages: Vec<String>,
  #[serde(default)]
  pub specifier_types: Vec<String>,
  //
  pub is_banned: Option<bool>,
  pub is_ignored: Option<bool>,
  pub pin_version: Option<String>,
  pub policy: Option<String>,
  pub snap_to: Option<Vec<String>>,
  pub prefer_version: Option<String>,
}

/// Resolve keywords such as `$LOCAL` and `!$LOCAL` to their actual values.
fn with_resolved_keywords(dependency_names: &[String], packages: &Packages) -> Vec<String> {
  let mut resolved_dependencies: Vec<String> = vec![];
  for dependency_name in dependency_names.iter() {
    match dependency_name.as_str() {
      "$LOCAL" => {
        for package in packages.all.iter() {
          let package_name = package.borrow().get_name_unsafe();
          resolved_dependencies.push(package_name);
        }
      }
      "!$LOCAL" => {
        for package in packages.all.iter() {
          let package_name = package.borrow().get_name_unsafe();
          resolved_dependencies.push(format!("!{}", package_name));
        }
      }
      _ => {
        resolved_dependencies.push(dependency_name.clone());
      }
    }
  }
  resolved_dependencies
}
