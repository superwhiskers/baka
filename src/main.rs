//
// baka - bypass the yandere simulator discord's mission mode verification
// Copyright (C) 2020 superwhiskers <whiskerdev@protonmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//

use std::{thread, time::Duration};
use discord_rpc_client::Client;

fn generate_activity_phase(client: &mut Client, state: &str) -> Result<(), Box<dyn std::error::Error>> {
    client.set_activity(|a| a
        .state(state)
        .details("Senpai... will... be... mine...")
        .assets(|a| a
            .large_text("This might be the game's box art one day!")
            .large_image("boxart")))?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rpc_client = Client::new(560185502691491841);

    rpc_client.start();

    println!("beginning verification routine. send `-verifyme` to MidoriBot#2780 within 30 seconds for this to work");

    generate_activity_phase(&mut rpc_client, "At the title screen!")?;
    thread::sleep(Duration::from_secs(30));

    generate_activity_phase(&mut rpc_client, "Accepting a mission!")?;
    thread::sleep(Duration::from_secs(10));

    generate_activity_phase(&mut rpc_client, "At School")?;
    thread::sleep(Duration::from_secs(5));

    for m in 0..12 {
        generate_activity_phase(&mut rpc_client, &format!("At School, 7:{:0>2} AM, Before Class, Monday, Mission Mode", m))?;
        thread::sleep(Duration::from_secs(20));
    }

    generate_activity_phase(&mut rpc_client, "Awaiting Verification")?;
    thread::sleep(Duration::from_secs(30));

    println!("verification routine finished. assuming everything was done properly, you should now be verified");

    Ok(())
}
