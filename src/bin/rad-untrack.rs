use librad::crypto::keystore::crypto::{KdfParams, Pwhash};
use librad::crypto::keystore::pinentry::Prompt;
use librad::git::tracking::git::tracking;

use rad_common::{keys, profile};
use rad_terminal::components as term;
use rad_track::options::Options;
use rad_untrack::HELP;

fn main() {
    term::run_command::<Options>(HELP, "Untracking", run);
}

/// Create a [`Prompt`] for unlocking the key storage.
pub fn prompt() -> Pwhash<Prompt<'static>> {
    let prompt = Prompt::new("please enter your passphrase: ");
    Pwhash::new(prompt, KdfParams::recommended())
}

fn run(options: Options) -> anyhow::Result<()> {
    term::info!(
        "Removing tracking relationship for {}...",
        term::format::highlight(&options.urn)
    );

    let profile = profile::default()?;
    let sock = keys::ssh_auth_sock();
    let (_, storage) = keys::storage(&profile, sock)?;

    if let Some(peer) = options.peer {
        tracking::untrack(
            &storage,
            &options.urn,
            peer,
            tracking::policy::Untrack::MustExist,
        )??;

        term::success!("Tracking relationship {} removed for {}", peer, options.urn);
    } else {
        tracking::untrack_all(&storage, &options.urn, tracking::policy::UntrackAll::Any)?
            .for_each(drop);

        term::success!("Tracking relationships for {} removed", options.urn);
    }

    Ok(())
}