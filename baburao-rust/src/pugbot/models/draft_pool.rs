use serenity::model::channel::{Embed, EmbedFooter};
use serenity::model::user::User;
use serenity::utils::Colour;
use std::collections::HashMap;
use typemap::Key;

use crate::queue_size;
use crate::traits::has_members::HasMembers;
use crate::traits::pool_availability::*;

pub struct DraftPool {
  pub members: Vec<User>,
  pub available_players: HashMap<usize, User>,
  pub max_members: u32,
}

impl DraftPool {
  pub fn new(members: Vec<User>, max_members: u32) -> DraftPool {
    DraftPool {
      members: members,
      available_players: HashMap::new(),
      max_members: max_members,
    }
  }

  pub fn available_players(self) -> HashMap<usize, User> {
    self.available_players
  }

  pub fn generate_available_players(&mut self) {
    for (idx, member) in self.members.clone().iter().enumerate() {
      self.available_players.insert(idx + 1, member.clone());
    }
  }

  pub fn pop_available_player(
    &mut self,
    player_number: &usize,
  ) -> Option<User> {
    self.available_players.remove(player_number)
  }
}

impl PoolAvailability for DraftPool {
  fn is_open(&self) -> bool {
    (self.members().len() as u32) < queue_size()
  }

  fn members_full_embed(&mut self, r: u8, g: u8, b: u8) -> Option<Embed> {
    let members = self.members();

    Some(Embed {
      author: None,
      colour: Colour::from_rgb(r, g, b),
      description: Some(members.into_iter().map(|m| m.clone().name).collect()),
      footer: Some(EmbedFooter {
        icon_url: None,
        proxy_icon_url: None,
        text: format!("The queue is full! Now picking captains!"),
      }),
      fields: Vec::new(),
      image: None,
      kind: "rich".to_string(),
      provider: None,
      thumbnail: None,
      timestamp: None,
      title: Some("Members in queue:".to_string()),
      url: None,
      video: None,
    })
  }
}

impl HasMembers for DraftPool {
  fn members(&self) -> Vec<User> {
    self.members.clone()
  }

  fn add_member(&mut self, user: User) -> Option<Embed> {
    self.members.push(user);
    self.members.dedup();

    if (self.members.len() as u32) == queue_size() {
      return self.members_full_embed(165, 255, 241);
    }

    self.members_changed_embed(165, 255, 241)
  }

  fn remove_member(&mut self, user: User) -> Option<Embed> {
    self.members.retain(|m| m.id != user.id);
    self.members.dedup();
    self.members_changed_embed(165, 255, 241)
  }

  fn members_changed_embed(&mut self, r: u8, g: u8, b: u8) -> Option<Embed> {
    let members = self.members.clone();

    Some(Embed {
      author: None,
      colour: Colour::from_rgb(r, g, b),
      description: Some(members.into_iter().map(|m| m.clone().name).collect()),
      footer: Some(EmbedFooter {
        icon_url: None,
        proxy_icon_url: None,
        text: format!(
          "{} of {} users in queue",
          self.members.len(),
          queue_size()
        ),
      }),
      fields: Vec::new(),
      image: None,
      kind: "rich".to_string(),
      provider: None,
      thumbnail: None,
      timestamp: None,
      title: Some("Members in queue:".to_string()),
      url: None,
      video: None,
    })
  }
}

impl Key for DraftPool {
  type Value = DraftPool;
}
