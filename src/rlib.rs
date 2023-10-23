use std::io;
use std::io::Write;

pub enum Command {
    Look(String),
    Go(String),
    Forward,
    Help,
    Quit,
    Unknown(String),
}

pub fn parse(input_str: String) -> Command {
    let lc_input_str = input_str.to_lowercase();
    let mut split_input_iter = lc_input_str.trim().split_whitespace();

    let verb = split_input_iter.next().unwrap_or_default().to_string();
    let noun = split_input_iter.next().unwrap_or_default().to_string();

    match verb.as_str() {
        "help" => Command::Help,
        "look" => Command::Look(noun),
        "go" => Command::Go(noun),
        "quit" => Command::Quit,
        "forward" => Command::Forward,
        _ => Command::Unknown(input_str.trim().to_string()),
    }
}

pub fn get_input() -> Command {
    // Prompt
    println!("");
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input_str = String::new();

    io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read move");
    println!("");

    // Parse & Return
    parse(input_str)
}

pub fn update_screen(output: String) {
    println!("{}", output);
}

pub struct Location {
    pub name: String,
    pub description: String,
}

pub struct World {
    pub player_location: usize,
    pub locations: Vec<Location>,
}

impl World {
    pub fn new() -> Self {
        World {
            player_location: 0,

            locations: vec![
                Location {
                    name: "Powhatan Wars".to_string(),

                    description: "Powhatan Chief Opechancanough led an attack that left nearly 350 of some 1,200 colonists dead. The English retaliated, attacking Native American villages, raiding and destroying crops and forcing them from their land".to_string(),
                },
                Location {
                    name: "Pequot War".to_string(),

                    description: "A series of conflicts, including a May 26, 1637, attack by English militia and Narragansett and Mohegan tribe members against the Pequot at Mystic".to_string(),
                },
                Location {
                    name: "Beaver Wars".to_string(),

                    description: "Bloody 60-year fur trade battle led by the Iroquois Confederacy of Ohio's St. Lawrence River against French-supported Algonquian-speaking tribes".to_string(),
                },
                Location {
                    name: "King Philip's War".to_string(),

                    description: "Starting with English encroachment, Metacom and his coalition retaliated, attacking colonists and settlements, who were aided by the Mohawks and Mohegans. It ended with Metacom’s death on August 12, 1676.".to_string()
                },
                Location {
                    name: "Pueblo Revolt".to_string(),

                    description: "The Pueblos, led by Po-Pay, sieged Santa Fe, destroying Spanish settlements and forcing the colonists into a 12-year retreat.".to_string()
                },
                Location {
                    name: "King William's War".to_string(),

                    description: "In what's also called the First French and Indian War, England, allied with the Iroquois Confederacy, battled France, with support from the Wabanaki Confederacy.".to_string()
                },
                Location {
                    name: "Tuscarora War".to_string(),

                    description: "In what's called Colonial North Carolina's bloodiest war, the Iroquoian-speaking Tuscarora people and allies battled English colonists as the settlers sought to expand their territory".to_string()
                },
                Location {
                    name: "Yamasee War".to_string(),

                    description: "Rebelling against their former allies, the Yamasee tribe, aligned with the Catawba and others, attacked the English in South Carolina colonies, triggered by treaty violations, encroachments and fur trade battles.".to_string()
                },
                Location {
                    name: "French and Indian War".to_string(),

                    description: "The imperial conflict pitted France and its Native American allies against Great Britain, aligned with the Iroquois Confederacy.".to_string()
                },
                Location {
                    name: "Cherokee War".to_string(),

                    description: "Tensions between the Cherokee and their former British allies grew in the Carolinas, due to attacks by the settlers, betrayals and land encroachment.".to_string()
                },
                Location {
                    name: "Pontiac's Rebellion".to_string(),

                    description: "Starting in the Ohio River Valley and Great Lakes and spreading as far east as Virginia, British forces battled members of the Algonquian, Iroquoian, Muskogean and Siouan-speaking tribes over trade restrictions, land encroachment and more.".to_string()
                },
                Location {
                    name: "Lord Dunmore's War".to_string(),

                    description: "A key conflict of the war was the October 10, 1774, Battle of Point Pleasant, where the militia defeated the Shawnee in present-day West Virginia. The ensuing treaty led to land south of the Ohio River being ceded to the British. ".to_string()
                },
                Location {
                    name: "Chickamauga Cherokee Wars".to_string(),

                    description: "In a struggle to keep their territory, the Cherokee, led by Chief Dragging Canoe, fought against American settlers throughout the Revolutionary War, across Tennessee, Kentucky, Virginia, the Carolinas and Georgia.".to_string()
                },
                Location {
                    name: "Battle of Fallen Timbers".to_string(),

                    description: "Considered the final battle of the American Revolution, the U.S. military, led by Maj. Gen. Anthony Wayne, successfully fought off a confederacy of American Indians.".to_string()
                },
                Location {
                    name: "Battle of Tippencanoe".to_string(),

                    description: "With Maj. Gen. William Henry Harrison at the helm, U.S. forces fought the Shawnee Indians who attacked an American camp along the Tippecanoe River in central Indiana. The Shawnee, led by Laulewasikau (called “The Prophet”), the brother of Tecumseh, were defeated, although casualties were nearly equal.".to_string()
                },
                Location {
                    name: "First Seminole War".to_string(),

                    description: "With Gen. Andrew Jackson at the command, U.S. soldiers attacked the Florida and southern Georgia villages of Seminoles and Black Seminoles, free African Americans and runaway slaves, seeking control of the territory and to recapture slaves.".to_string()
                },
                Location {
                    name: "Arikara War".to_string(),

                    description: "The first Plains Indian War, took place west of the Missouri River between the semi-nomadic Arikara of South Dakota and the U.S, which was joined by Sioux allies. Facing encroachment, the Arikara had attacked members of the Rocky Mountain Fur Company, killing about 15 trappers.".to_string()
                },
                Location {
                    name: "Second Seminole War".to_string(),

                    description: "A dispute over terms of a treaty to relocate Seminoles from their established reservation near Lake Okeechobee in Florida caused a drawn-out, bloody war of resistance that ended with the loss of 1,500 U.S. soldiers and 3,000 Seminoles removed from their land, with the fewer than 500 remaining left to die in the Everglades. ".to_string()
                },
                Location {
                    name: "Third Seminole War".to_string(),

                    description: "Also called Billy Bowlegs' War, as the Seminole leader was called, the war a series of attacks and raids over land, eventually causing the tribe's population to shrink to around 200 by the conflict's end. ".to_string()
                },
                Location {
                    name: "Black Hawk War".to_string(),

                    description: "Sauk warrior Chief Black Hawk, leading approximately 1,500 Sauk, Kickapoo and Meskwaki tribal members across the Mississippi River from Iowa to reclaim surrendered land in Illinois, lead to an attack by the U.S. Army, allied with other tribes and state militias.".to_string()
                },
                Location {
                    name: "Comanche War".to_string(),

                    description: "The high plains Comanche tribe of Central and West Texas had fought against frontiersmen for years, but facing smallpox, war with the Arapaho and Cheyenne and conflicts with the Texas Rangers, leaders agreed to enter treaty negotiations.".to_string()
                },
                Location {
                    name: "Sand Creek Massacre".to_string(),

                    description: "In the midst of the Civil War and a long-waging battle for control of eastern Colorado's Great Plains, a unit of some 675 volunteer U.S. soldiers laid siege against a Cheyenne and Arapaho village of approximately 750 along Sand Creek. As the American Indians attempted to escape, the soldiers charged after them, slaughtering 230, most of whom were unarmed women, children and elderly. Soldiers returned the next day to scalp and desecrate their bodies. ".to_string()
                },
                Location {
                    name: "Red Cloud’s War".to_string(),

                    description: "Following the discovery of gold in Montana, pioneer John Bozeman blazed the Bozeman Trail, built on American Indian territory in the Wyoming Territory. Angered by the encroachment and presence of U.S. forces, and a failed treaty meeting, Red Cloud, leader of the Oglala, teamed with Arapaho and Cheyenne, including Crazy Horse and High-Back-Bone, to ambush travelers along the trail, as well as U.S. military. ".to_string()
                },
                Location {
                    name: "Battle of the Little Bighorn".to_string(),

                    description: "After an Army expedition led by Lt. Col. George Custer discovered gold in the Black Hills of South Dakota, within the Great Sioux Reservation, and offers to sell the territory were rejected by the Sioux Nation, the Treaty of Fort Laramie was quickly abandoned. Custer, with 209 men, attacked a Sioux, Arapaho and Cheyenne encampment in Montana's Little Bighorn River, resulting in the deaths of Custer and all his men. ".to_string()
                },
                Location {
                    name: "Red River War".to_string(),

                    description: "In the final major southern Plains Indian and U.S. Army battle, members from a number of Indian tribes, including Arapaho, Cheyenne, Comanche, Kiowa and Kataka, who had been settled on reservations in Oklahoma and Texas, broke away to attack white settlers. In response, some 3,000 troops led by Gen. William Tecumseh Sherman, attacked up to 700 Indians in Texas's Red River valley in more than a dozen battles before the surviving Indians surrendered, returning to their reservations. ".to_string()
                },
                Location {
                    name: "Wounded Knee Massacre".to_string(),

                    description: "Two weeks after Sitting Bull was killed by Indian police, hundreds of unarmed Kakota Sioux men, women and children were massacred by Army soldiers who had arrived at the Pine Ridge Reservation in South Dakota to stop Tribal members from performing Ghost Dance rituals. One of the final U.S. military actions against northern Plains Native Americans, 20 medals of honor were awarded to soldiers who took part in the carnage.".to_string()
                },
                Location {
                    name: "20th Century".to_string(),

                    description: "Small amounts of fighting continues on to the 1920's.".to_string()
                },
                Location {
                    name: "American Indian Movement Founded".to_string(),

                    description: "The modern American-Indian movement is startint to take shape, with decades of systemic opression with little resistance taking its toll. Issues it was founded because included treaty rights, high rates of unemployment, the lack of American Indian subjects in education, and the preservation of Indigenous cultures".to_string()
                },
            ],
        }
    }

    pub fn update_state(&mut self, command: &Command) -> String {
        match command {
            Command::Look(noun) => self.do_look(noun),
            Command::Go(noun) => self.do_go(noun),
            Command::Forward => self.forward(),
            Command::Help => format!("How To Play\n\nhelp - This message\nforward - Travel forward in time\nlook - Prints information about whats happening now\ngo - Change the time and place\nquit - Quit the game"),
            Command::Quit => format!("Quitting.\nThank you for playing!"),
            Command::Unknown(input_str) => format!("I don't know how to '{}'.", input_str),
        }
    }
    pub fn do_look(&self, noun: &String) -> String {
        match noun.as_str() {
            "around" | "" => format!(
                "{}\n\n{}.\n",
                self.locations[self.player_location].name,
                self.locations[self.player_location].description
            ),
            _ => format!("I don't understand what you want to see.\n"),
        }
    }

    pub fn forward(&mut self) -> String {
        let mut output = String::new(); 

        let pos = self.player_location + 1;
        if pos < self.locations.len() {
            self.player_location = pos;
            output = output + &format!("You travel a few years into the future.");
        } else {
            output = output + &format!("You have reached the modern day. Try using look to get to previous times.");
        };
        output

    }


    pub fn do_go(&mut self, noun: &String) -> String {
        let mut output = String::new();

        for (pos, location) in self.locations.iter().enumerate() {
            if *noun == location.name.to_lowercase() {
                if pos == self.player_location {
                    output = output + &format!("Wherever you go, there you are.\n");
                } else {
                    self.player_location = pos;
                    output = output + &format!("OK.\n\n") + &self.do_look(&"around".to_string());
                }
                break;
            }
        }

        if output.len() == 0 {
            format!("I don't understand where you want to go.")
        } else {
            output
        }
    }
    // ....
}
