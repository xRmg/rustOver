# Name
PushOver client in Rust - In progress

# Usage
<pre>
rustover -t <token> -u <user> -m <message>
-t <token>			- (Required) Pushover Api token (30 characters)
-u <user_key> 		- (Required) Pushover user/group key of your user (30 characters)
-m <message>		- (Required) the message to be send.

-device <devices>	- user's device name to send the message directly to (multiple devices may be separated by a comma)
-title <title>		- message's title, otherwise your app name is used
-url <url>			- supplementary URL to show with your message
-url_title <title>	- a title for your supplementary URL, otherwise just the URL is shown
-priority <n>		- 	-2 to generate no notification/alert, 
						-1 to always send as a quiet notification, 
						1 to display as high-priority and bypass the user's quiet hours
						2 to also require confirmation from the user
-timestamp <t_unix>	- Unix timestamp of your message's date and time to display to the user.
-sound <sound_name>	- the name of one of the sounds supported by device 
-use-html			- this will allow html to be posted in the message. 
</pre>
