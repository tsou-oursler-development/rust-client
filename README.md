# Rust IRC Client
**Authors: Briana Oursler and Lily Tsou**
<hr>
Rust IRC Client is a text-based user interface client that allows users to communicate with each other over a network via the IRC protocol. It was written entirely in 
Rust and uses the <a href="https://github.com/gyscos/cursive">cursive</a> and <a href="https://github.com/aatxe/irc">irc</a> crates.<br>
Users running this client are prompted to enter a server, channel, and name. If the server and channel exist, then NAME will connect the user
to the channel with their chosen nickname. <br>
All messages submitted to the tui by a user are sent through a channel by a std::sync::mpsc::Sender. The main thread receives the messages with a 
std::sync::mpsc::Receiver and appends them to the chat window on the tui. The main thread also sends the messages to the controller, which uses the
IRC API to send the messages to the server. Messages are received from the server by the controller, which uses the same Sender as the tui to send the 
messages through the channel to main, which then appends them to the chat window.
<hr>
<h2> How to Build and Run </h2>
After cloning this repository, use "cargo run" to open the client. <br>
A window will appear asking for login credentials: a server and channel to connect to, and the name you wish to appear as your nickname. <br>
<img src="https://user-images.githubusercontent.com/77073427/121328292-0e5eaf00-c8c9-11eb-9411-f2c44c37959d.PNG"
     	alt="Image of login screen"
     	style="float: left; margin-right: 10px;" />
Once the credentials have been entered, if the server and channel are valid then a window will appear with a log from the server and a text editor for 
entering new chat messages. <br>
Users are able to send messages to either the entire channel, or to specific users. If no channel is specified, then the client will assume that the 
user is attempting to send a message to a user, and will take the first word of their message to be the recipient's nickname. A channel is specified by
using the '#' symbol.<br>
For example, <br>
#test-channel hello <br>
will send a message to the channel 'test-channel'.<br>
test-user hello <br>
will send a message to the user 'test-user'.<br>
<img src="https://user-images.githubusercontent.com/77073427/121330433-e07a6a00-c8ca-11eb-952f-291ad0d4e7a1.PNG"    
     	alt="Image of login screen"
	style="float: left; margin-right: 10px;" />
The chat window is set to continue scrolling to the bottom as messages appear, however this feature is overriden when a user uses their cursor instead of their 
keyboard to send a message. In this case, the user must manually scroll to the bottom to see new messages.
<br>
<hr>
<h2> Known Issues </h2>
As mentioned above, cursor events seem to remove the auto-scroll functionality. If you only navigate the chat window with the keyboard, then all new messages are 
displayed as they are received and the focus stays on the bottom of the chat window. However, a single mouse event causes the focus to stay at the current position 
and all new messages are appended below the current window view. <br>
The quit command logs the user out, but does not close the tui. This is because the quit event is received via the main thread, and a cursive object cannot be
moved into it. Therefore, it is not possible to call tui.quit(), and the thread waits for the next event before closing the tui. <br>
<hr>
<h2> Looking Forward </h2>
Though this project is not completely free of bugs, we are still very pleased with how it turned out. Considering the fact that neither of us even knew where to start 10 weeks ago, 
we feel as though we have accomplished quite a bit! We both learned a great deal about the crates we used, and though we needed help connecting the controller 
and view, this was also a learning process which taught us a great deal about Rust channels. There have been large architectural rewrites and many lines of 
abandoned code, but the final project is a working IRC client. <br>
Future goals for this project would be to work on more message parsing to allow for extended user functionality and to work on the tui bugs mentioned above.<br>
We would also like to add a drop-down menu of existing channels for a user to connect to and some sort of warning if the user has selected an invalid server in the login process.<br>
<hr>
<h2> License </h2>
LICENSE LINK.
