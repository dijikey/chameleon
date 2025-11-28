---@meta
---@diagnostic disable

-- Toolkit for "chameleon" library
---@type table
toolkit = {}
local color = {};
---@class Status
local status = nil;

---Create color table with r,g,b,a
--- @param r number In range [0, 255]
--- @param g number In range [0, 255]
--- @param b number In range [0, 255]
--- @param a number In range [0, 100]
--- @return Color
function color.rgba(r,g,b,a) end
--- Create color table with r,g,b
--- @param r number In range [0, 255]
--- @param g number In range [0, 255]
--- @param b number In range [0, 255]
--- @return Color
function color.rgb(r,g,b) end


--- Return TRUE if status is disable
--- @param status Status
--- @return boolean
function toolkit.is_disabled(status) end
--- Return TRUE if status is active
--- @param status Status
--- @return boolean
function toolkit.is_active(status) end
--- Return TRUE if status is hovered
--- @param status Status
--- @return boolean
function toolkit.is_hovered(status) end
--- Return TRUE if status is pressed
--- @param status Status
--- @return boolean
function toolkit.is_pressed(status) end
--- Return TRUE if status is dragged
--- @param status Status
--- @return boolean
function toolkit.is_dragged(status) end
--- Return TRUE if status is idle
--- @param status Status
--- @return boolean
function toolkit.is_idle(status) end


---@class Color
local class = {
    ---@type number In range [0, 255]
    r=0,
    ---@type number In range [0, 255]
    g=0,
    ---@type number In range [0, 255]
    b=0,
    ---@type number In range [0, 100]
    a=0
};

---@class Rounded
local class = {
    --- @type number
	top_left=nil,
	--- @type number
	top_right=nil,
	--- @type number
    bottom_right=nil,
	--- @type number
	bottom_left=nil
};

---@class Border
local class = {
    --- @type Color
    color=nil,
    --- @type number
    width=nil,
    --- @type Rounded
    rounded=nil
};

---@class Vector
local class = {
    --- @type number
    x=nil,
    --- @type number
    y=nil
};

---@class Shadow
local class = {
    --- @type Color
    color=nil,
    --- @type number
    blur_radius=nil,
    --- @type Vector
    offset=nil
};

---@class Button
local class = {
    ---@type Color|nil
    background=nil,
    ---@type Color
    text_color=nil,
    ---@type Border
    border=nil,
    ---@type Shadow
    shadow=nil
};

---@class Appearance
local class = {
    ---@type Color
    background=nil,
    ---@type Color
    text_color=nil,
};

---@class Text
local class = {
    ---@type Color|nil
    color=nil
}

---@class TextInput
local class = {
    --- @type Color|nil
	background = pallete.WHITE,
	--- @type Border
	border = style.border(),
	--- @type Color
	icon = pallete.WHITE,
	--- @type Color
	placeholder = pallete.WHITE,
	--- @type Color
	value = pallete.WHITE,
	--- @type Color
	selection = pallete.WHITE,
}

---@class Svg
local class = {
    ---@type Color|nil
    color=nil
}

---@class Container
local class = {
	--- @type Color|nil
	background = nil,
	--- @type Color|nil
	text_color = nil,
	--- @type Border
	border = style.border(),
	--- @type Shadow
	shadow = style.shadow(),
}

---@class Highlight
local class = {
	--- @type Color
	background = pallete.WHITE,
	---@type Border
	border = style.border(),
}

---@class Line
local line = {
	--- @type Color
	color = pallete.WHITE,
	---@type number float
	width = 0,
}

---@class PaneGrid
local pane_grid = {
	--- @type Highlight
	hovered_region = style.highlight(),
	--- @type Line
	picked_split = style.line(),
	--- @type Line
	hovered_split = style.line()
}

---@class Rule
local rule = {
	--- @type Color
	color = pallete.WHITE,
	--- @type number number > 0
	width = 0,
	--- @type Rounded
	rounded = style.rounded(0)
}

---@class Scroller
local scroller = {
	--- @type Color
	color = pallete.WHITE,
	--- @type Border
	border = style.border()
}

---@class Rail
local rail = {
	--- @type Color|nil
	background = nil,
	--- @type Border
	border = style.border(),
	--- @type Scroller
	scroller = style.scroller()
}

---@class Scrollable
local scroll = {
	--- @type Color|nil
	gap = nil,
	--- @type Container
	container = style.container(),
	--- @type Rail
	rail = style.rail()
}

toolkit.color = color