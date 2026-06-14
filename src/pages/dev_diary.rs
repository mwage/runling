use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

#[derive(Clone, Copy)]
struct DiaryEntry {
    id: &'static str,
    date: &'static str,
    title: &'static str,
    description: &'static str,
}

const DIARY_ENTRIES: &[DiaryEntry] = &[
    DiaryEntry {
        id: "002",
        date: "October 20, 2017",
        title: "Chat, Friends and Rooms",
        description: "Preview of how Chat-, Friend- and Roomsystem are implemented in our game.",
    },
    DiaryEntry {
        id: "001",
        date: "August 28, 2017",
        title: "Update on Multiplayer",
        description: "A quick glance at different multiplayer concepts and how they apply to our Runling game!",
    },
];

#[component]
pub fn DevDiaryList() -> impl IntoView {
    view! {
        <div class="dev-diary">
            <Title text="Developer Diary - Runling"/>
            <h2>"Developer Diary"</h2>
            {DIARY_ENTRIES
                .iter()
                .map(|entry| {
                    view! {
                        <div class="row news-item">
                            <A href=format!("/dev-diary/{}", entry.id)>
                                <div class="col-md-3 col-sm-5 col-xs-12">
                                    <img
                                        class="img-responsive"
                                        src=format!("images/dev-diary/{}.jpg", entry.id)
                                        alt=entry.title
                                    />
                                </div>
                                <div class="col-md-9 col-sm-7 col-xs-12">
                                    <h4>{entry.title}</h4>
                                    <p class="article">{entry.description}</p>
                                    <p class="date">{entry.date}</p>
                                </div>
                            </A>
                        </div>
                    }
                })
                .collect_view()}
        </div>
    }
}

#[component]
pub fn DevDiary001() -> impl IntoView {
    view! {
        <div>
            <Title text="Update on Multiplayer - Runling"/>
            <h2>"Update on Multiplayer"</h2>
            <div class="article">
                <div class="logo">
                    <img
                        class="img-responsive"
                        src="images/dev-diary/001.jpg"
                        alt="Update on Multiplayer"
                    />
                </div>
                <p>"Hihi, since it's been a while, I thought I'd share an update on how things are progressing. First a quick summary:"</p>
                <p>
                    "Runling is actually one of the more tricky games to write a multiplayer for (more on that later). I tried out 2 Backend as a Service (BaaS) network solutions (first photon unity networking, then their TrueSync solution) to get a functioning prototype for the game and while both were somewhat working, none of them was really solid enough for play-testing and both were already showing major flaws for our type of game. Therefore I decided to abandon the idea of getting a quick prototype out and instead will be setting up everything for a dedicated server. This will take me a while to get it to a playable state, so until then, you'll have to be satisfied with update posts like this one."
                </p>
                <p>
                    "In the next section I'll go more into detail about multiplayer concepts, how they apply to Runling and things I've learned about game networking over the past 2 months. If you're not interested in that, skip to the end to the 2 video links (login, room system + difficulty selection)."
                </p>
                <br/>
                <h3>"Peer-to-Peer"</h3>
                <p>
                    "P2P means every player is connected with each other. This is arguably the easiest for prototyping, since you don't need a server. This is usually where BaaS shines. There are several network solutions to pick from, an example would be Photon Unity Networking (PUN), which was the first one I've been working with. One of the connected clients is the host (or master client). He basically replaces the server, handles most of the game logic and sends the necessary data to the other players. General disadvantages are that it's hard to prevent cheating (not really a main point of concern at the moment for me) and that you are kinda dependent on the hosts connection speed."
                </p>
                <p>
                    "Overall that doesn't sound too bad, but there's another big problem for a game like ours: When you have hundreds of neutral game objects (drones here) that need to be synchronized for each player, you run into a lot of problems. How do you deal with them? Do you just let them all run at the host and he's then sending their position to other clients? Not only would that give the host the advantage of having 0 ping, he'd also need to send this data to every connected client, which would result in unreasonable bandwidth usage. Do you check collisions for the players on the host side too? Usually you run player instances on the according client, but then the host that checks collisions only has an interpolated position of the player to compare against the drone that is on it's actual position, and so on..."
                </p>
                <p>"As you can see, it becomes kind of a mess. P2P might be great for prototyping for example an FPS, but for Runling it's sadly out."</p>
                <p>"Now let's  figure out how Starcraft 2 and other RTS handle the huge amount of units there while maintaining low bandwidth cost:"</p>
                <h3>"Lockstep"</h3>
                <p>
                    "In a basic lockstep system, once again all players are connected with each other like in P2P, but instead of exchanging all the data, all they are exchanging is inputs. The way that works is: every client runs the exact same code with a deterministic engine (an engine that always produces the same results if given the same input). So all you need to know from the other players is the input relevant for the game state (in our example things like mouse clicks, skill hotkeys, etc)."
                </p>
                <p>
                    "This makes it a decent choice whenever lots and lots of objects are used. However there are also disadvantages to it. Firstly, in a usual lockstep setup, on every lockstep tick, it waits for the input of every player before proceeding. This means, the player with the worst connection slows down everyone else. A way around it is using routed Lockstep (this is what Starcraft 2 uses), where they use a server to distribute all the input commands. If someone's having bigger lags, it will still slow down the game for everyone though to avoid desync and this brings us to the next big disadvantage."
                </p>
                <p>
                    "Desync Hell, that refers to small tiny tiny differences on one client can cause a butterfly effect that make you end up with a completely different game state. Creating lockstep games is a nightmare, everything has to be perfect, or it falls completely apart. You also need to have a deterministic engine (Unity, Unreal or pretty much any usually used game engine is not deterministic)."
                </p>
                <p>
                    "The 2nd solution i tried for prototyping Runling was Photons TrueSync, which is a Lockstep engine. Sadly the 3D-part engine is still pretty immature and rather inconsistent. I went through one bug after another and after trying to work around what felt like a dozen of game breaking bugs while making compromises all over, I gave up. It didn't feel worth my time just to get a crappy prototype out."
                </p>
                <p>
                    "To sum it up: Lockstep is definitely something that could be used for a Runling game (see Runling Run 4 in SC2), but would be very tedious to implement and maintain and pose the same problems RLR4 multiplayer has (artificial latency due to lockstep ticks, players lagging everyone else in the game, etc)."
                </p>
                <h3>"Client-Server"</h3>
                <p>
                    "Client-Server model is pretty straight forward and probably what most people think about when thinking multiplayer. You have a dedicated server that runs it's own server logic and the player then connects to it and receives all the data he needs. For example in our case, you have the basic server that handles things like logins, etc and when you start a game, it would create a game instance, which runs it's own (stripped down) simulation of the game."
                </p>
                <p>
                    "This obviously has huge benefits: Firstly you have an authoritative server that can handle all the game logic. Every object runs on the server (drones, players, etc) and every gameplay decision (like deaths f.e.) will be handled by it, making it very consistent and preventing cheating. Players send their input to the server and the server sends the current state of the game to everyone."
                </p>
                <p>
                    "As usual there are also downsides. If you want to synchronize hundreds of objects via server, you need to be very careful about what you send and how you send it, otherwise bandwidth usage will go beyond reasonable. There are multiple ways to compress the data and interpolate over it, so with some work, it should be possible to keep it at an acceptable rate. 10-20 years ago this probably wouldn't have been reasonable, but the internet has come a long way since then. This will still be one of the bigger challenges to deal with in our development."
                </p>
                <p>
                    "Another disadvantage are of course server costs. If you want to test with others instead of just locally, you'll have to set up a server (f.e. at AWS or Azure). Well, that's just something you have to deal with i guess, but for the time being, we'll just be testing locally until it feels worth setting one up."
                </p>
                <br/>
                <h3>"Conclusion"</h3>
                <p>
                    "I hope you enjoyed this small peak into the world of game networking. My plan is to use a Client-Server setup using Darkrift, which is a very lightweight and flexible network solution. It does the basic networking things for you, but not much more, so you have full control over what and how things get sent (and the event-driven messaging system it uses is quite comfortable to use). The downside is that you have to write almost everything yourself."
                </p>
                <br/>
                <h3>"Preview to Room-System, Arena selection and Login"</h3>
                <p>"Since I don't want to leave you without any updates about the game itself, I'll leave you with 2 videos:"</p>
                <p>"First the room system:"</p>
                <iframe
                    width="560"
                    height="315"
                    src="https://www.youtube.com/embed/9ISfyruJtJ8"
                    frameborder="0"
                    allowfullscreen
                ></iframe>
                <p>
                    "The logic behind it is still written for the Photon solution, so I'll still have to write server plugin and client logic for the Client-Server model, but generally this is what it'll be like (just more polished graphically and with things like chat added)."
                </p>
                <p>"Secondly the Login to the Server"</p>
                <iframe
                    width="560"
                    height="315"
                    src="https://www.youtube.com/embed/2oHDLnk_Dt0"
                    frameborder="0"
                    allowfullscreen
                ></iframe>
                <p>
                    "This is already done with Darkrift, it's a basic Login System. At first you can see the empty database that holds the player accounts, the command prompt is the Darkrift server. If you try to log in with a username/password combination that doesn't exist, you get and error. If you create a new user, it gets added to the database and logs you in. In the server prompt, you can now see the events and if you check the database, there's now an entry for the test user with his hashed password. If you log out, you get sent back to the login screen, where you can now log in with our login data."
                </p>
                <p>
                    "This concludes what I've been up to this summer. Not too much to actually show off, but I learned a lot from it, which will help for implementing a solid Multiplayer in the future. If you liked this kind of post (going a bit deeper behind the scenes), then let me know and I'll do more in the future! Next one would probably be about the leveling and skill system, for which the foundation is pretty much done."
                </p>
                <p>"That's it for today! Thanks for your time an have a nice day!"</p>
            </div>
            <p class="date">"August 28, 2017"</p>
        </div>
    }
}

#[component]
pub fn DevDiary002() -> impl IntoView {
    view! {
        <div>
            <Title text="Chat, Friends and Rooms - Runling"/>
            <h2>"Chat, Friends and Rooms"</h2>
            <div class="article">
                <div class="logo">
                    <img
                        class="img-responsive"
                        src="images/dev-diary/002.jpg"
                        alt="Chat, Friends and Rooms"
                    />
                </div>
                <p>
                    "Hello everyone, welcome to another dev-diary! This time I'd like to share a small preview about how Chat, \
                    Friendlist and Roomsystem are currently working in Runling. First off, here's a short video showcasing it:"
                </p>
                <iframe
                    width="560"
                    height="315"
                    src="https://www.youtube.com/embed/hlSaSQQN2PE"
                    frameborder="0"
                    allowfullscreen
                    style="margin-bottom: 10px"
                ></iframe>
                <p>"As you can see, it's heavily inspired by how it works in SC2."</p>
                <h3>"Friendlist"</h3>
                <p>
                    "As you can see you can send and accept/decline friend requests. It doesn't matter if the other person is \
                    online at the time of sending the request. If he is, he gets notified, otherwise he sees it on the next login."
                </p>
                <h3>"Roomsystem"</h3>
                <p>
                    "You can create a room (and choose a name for it), or join an existing one. If the host leaves, a new host \
                    gets assigned by the server. The colors are currently automatically assigned, server logic for choosing your own color is \
                    already done though, I just need to implement the dropdown on the client side. Lastly you can start the game, \
                    which sends everyone in the room the game server connection info and loads the scene."
                </p>
                <h3>"Chat"</h3>
                <p>
                    "There are 4 types of messages: Private messages that arekept between you and the receiving user, Room messages \
                    that gets distributed to everyone in the room, Server messages that are mostly used for notifications, errors, etc. \
                    and Group messages for everyone in said chatgroup. You can join a chatgroup by typing \"/join groupname\". \
                    If the group doesn't currently exist, it gets created. Creating a room is case sensitive, joining one is not. \
                    For example, you can create a group called RunLinG, but you can just join it with \"/join runling\", you'll get the \
                    real name send from the server. It saves our active groups, so on next login, you'll \
                    automatically join all the groups you were in before the logout. Lastly you can filter messages per channel \
                    or just show all, navigate between output channels with tab, open the chat window with enter and close chat/friend windows with escape."
                </p>
                <h3>"Login"</h3>
                <p>
                    "I rewrote the entire login plugin (due to upgrading to DR 2 and switching to MongoDb). It's also a lot more secure now. \
                    It now uses RSA encryption to get the password safely from the user to the server and Bcrypt to salt hash the passwords to the database. \
                    I might switch to using OAuth2 instead to save myself the hassle of storing passwords myself, \
                    but for the time being this is more than enough."
                </p>
                <hr/>
                <p>
                    "While I'm overall somewhat happy with how things work, I definitely want to change a few things: \
                    I want to use dropdowns to get options like removing friends or channels instead of \
                    the ugly red X buttons, group PMs in a dropdown and show the players that are currently in the chat group \
                    (once again in a dropdown). \
                    As you can see, they have something in common, which is also the reason i haven't implemented it yet: Dropdowns. \
                    I just didnt really use the unity dropdowns yet, so while it's probably rather easy, I just have to learn it. \
                    I also want to make the Lobby nicer, but all these things will have to wait a bit, since I kinda want a break from UI work. \
                    On the plus side, most serverside code for this is already done, so i really just need to implement it."
                </p>
                <p>
                    "Lastly, plans for what I wanna work on next: First I still need to properly redirect the players to the \
                    game server instance. Afterwards I want to implement the Arena Difficulty voting (since I already wrote \
                    that for a different network solution, it will be easy to rewrite). After that I want to finish up the \
                    4/5th done skill- and leveling system and then make a good plan on how to structure the game code for multiplayer. \
                    Since I want to keep an offline mode, I'll have to kind of split it in 2, but would like to reuse as much code as \
                    possible, so I gotta come up with a good way to approach this."
                </p>
                <p>
                    "As usual, thanks for reading and feel free to leave your comments on the message board or in the old forums. \
                    Let me know what else you'd like to see in a chatsystem like this."
                </p>
                <p>
                    "PS: If you ever want to add multiplayer to a game of yours, I made a template project with pretty much these features. \
                    You can get it "
                    <a
                        href="https://github.com/blubwage/DarkRift2_Boilerplate"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        "here!"
                    </a>
                </p>
            </div>
            <p class="date">"October 20, 2017"</p>
        </div>
    }
}
