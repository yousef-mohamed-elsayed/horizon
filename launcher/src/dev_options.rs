//! Developer options, revealed by seven clicks on the launcher's version.

use super::*;

const CLICKS: u8 = 7;

impl Launcher {
    pub(super) fn version_clicked(&mut self) {
        if self.dev_options {
            return;
        }
        self.dev_clicks += 1;
        if self.dev_clicks >= CLICKS {
            self.dev_options = true;
            self.go(Page::Settings);
        }
    }

    pub(super) fn dev_options_section(&self, cx: &mut Context<Self>) -> Option<Div> {
        if !self.dev_options && !self.settings.dev_offline {
            return None;
        }
        let t = self.theme;
        let toggle = switch("dev-offline", self.settings.dev_offline, &t).on_click(cx.listener(
            |this, _: &ClickEvent, _, cx| {
                this.settings.dev_offline = !this.settings.dev_offline;
                this.save_settings();
                cx.notify();
            },
        ));
        let profiles = self
            .accounts
            .offline
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, name)| {
                let selected = self.accounts.name_arg() == Some(name.as_str());
                let label = name.clone();
                let action = button(
                    format!("dev-offline-profile-{index}"),
                    if selected { tr("Selected") } else { tr("Select") },
                    Kind::Secondary,
                    &t,
                )
                .on_click(cx.listener(move |this, _: &ClickEvent, _, cx| {
                    this.accounts.selected = None;
                    this.accounts.offline_selected = Some(name.clone());
                    if let Err(error) = this.accounts.save() {
                        this.client_error = Some(error);
                    }
                    cx.notify();
                }));
                index_row(
                    index,
                    div().child(label),
                    None,
                    Some(action.into_any_element()),
                    &t,
                )
            });
        Some(
            div()
                .flex()
                .flex_col()
                .child(
                    section_title("Developer options", "terminal", &t)
                        .mt(px(20.))
                        .mb(px(12.)),
                )
                .child(index_row(
                    0,
                    tr("Offline profiles without an account"),
                    Some(
                        div()
                            .child(tr(
                                "Debug builds can start as a saved offline profile without a Microsoft account that owns the game. Game files must already be installed.",
                            ))
                            .into_any_element(),
                    ),
                    Some(toggle.into_any_element()),
                    &t,
                ))
                .when(!self.accounts.offline.is_empty(), |d| {
                    d.child(section_title("Offline profiles", "user", &t).mt(px(16.)))
                        .children(profiles)
                }),
        )
    }
}