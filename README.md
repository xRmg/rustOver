# Name
PushOver client in Rust - In progress

# Usage
<pre>
rustover -t  &lt;token &gt; -u  &lt;user &gt; -m  &lt;message &gt;
-t  &lt;token &gt;              - (Required) Pushover Api token (30 characters)
-u  &lt;user_key &gt;           - (Required) Pushover user/group key of your user (30 characters)
-m  &lt;message &gt;            - (Required) the message to be send.

-device  &lt;devices &gt;       - user's device name to send the message directly to (multiple devices may be separated by a comma)
-title  &lt;title &gt;          - message's title, otherwise your app name is used
-url  &lt;url &gt;              - supplementary URL to show with your message
-url_title  &lt;title &gt;      - a title for your supplementary URL, otherwise just the URL is shown
-priority  &lt;n &gt;           - -2 to generate no notification/alert, 
                          -1 to always send as a quiet notification, 
                           1 to display as high-priority and bypass the user's quiet hours
                           2 to also require confirmation from the user
-timestamp  &lt;t_unix &gt;     - Unix timestamp of your message's date and time to display to the user.
-sound  &lt;sound_name &gt;     - the name of one of the sounds supported by device 
-use-html               - this will allow html to be posted in the message. 
</pre>

#License
The MIT License (MIT)
