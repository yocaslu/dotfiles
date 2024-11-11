#!/bin/bash
# xmodmap -e "keycode 66 = Mode_switch" &&
# xmodmap -e "keysym h = h H Left" &&
# xmodmap -e "keysym j = j J Down" &&
# xmodmap -e "keysym k = k K Up" &&
# xmodmap -e "keysym l = l L Right"  

# Notebook Maps
xmodmap -e "keycode 66 = Mode_switch" &&
xmodmap -e "keysym j = j J Left" &&
xmodmap -e "keysym k = k K Down" &&
xmodmap -e "keysym l = l L Up" &&
xmodmap -e "keysym ccedilla = ccedilla Ccedilla Right"

# Desktop Maps 
# xmodmap -e "keycode 66 = Mode_switch" &&
# xmodmap -e "keysym j = j J Left" &&
# xmodmap -e "keysym k = k K Down" &&
# xmodmap -e "keysym l = l L Up" &&
# xmodmap -e "keysym semicolon = semicolon colon  Right"
