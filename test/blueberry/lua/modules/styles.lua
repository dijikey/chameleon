local pallete = require("pallete")

---@class Style
local style = {}

---@param value number
style.rounded = function (value)
	---@class Rounded
	local rounded = {
		--- @type number
		top_left=value,
		--- @type number
		top_right=value,
		--- @type number
		bottom_right=value,
		--- @type number
		bottom_left=value
	}

	return rounded
end

style.border = function ()
	---@class Border
	local border = {}
	--- @type Color
	border.color = pallete.TRANSPARENT
	--- @type number
    border.width = 0
	--- @type Rounded
	border.rounded = style.rounded(0)

	return border
end

style.vector = function (x, y)
	---@class Vector
	local vector = {}
	--- @type number float
	vector.x = x
	--- @type number float
	vector.y = y

	return vector
end

style.shadow = function ()
	---@class Shadow
	local shadow = {}
	--- @type Color
	shadow.color = pallete.TRANSPARENT
	--- @type number float
	shadow.blur_radius = 0
	--- @type Vector float
	shadow.offset = style.vector(0, 0)

	return shadow
end

style.button = function ()
	---@class Button
	local button = {}
	--- @type Color|nil
	button.background = nil
	--- @type Color
	button.text_color = pallete.WHITE
	--- @type Border
	button.border = style.border()
	--- @type Shadow
	button.shadow = style.shadow()

	return button
end


style.appearance = function ()
	---@class Appearance
	local button = {}
	--- @type Color
	button.background = pallete.WHITE
	--- @type Color
    button.text = pallete.WHITE
    --- @type Color
    button.primary = pallete.WHITE

	return button
end

style.text = function ()
	---@class Text
	local text = {
		--- @type Color|nil
		color = nil
	}
	return text
end

style.text_input = function ()
	---@class TextInput
	local text_input = {
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

	return text_input
end

style.svg = function ()
	---@class Svg
	local svg = {
		--- @type Color|nil
		color = nil
	}

	return svg
end

style.container = function ()
	---@class Container
	local container = {
		--- @type Color|nil
		background = nil,
		--- @type Color|nil
		text_color = nil,
		--- @type Border
		border = style.border(),
		--- @type Shadow
		shadow = style.shadow(),
	}

	return container
end

style.highlight = function ()
	---@class Highlight
	local highlight = {
		--- @type Color
		background = pallete.WHITE,
		---@type Border
		border = style.border(),
	}

	return highlight
end

style.line = function ()
	---@class Line
	local line = {
		--- @type Color
		color = pallete.WHITE,
		---@type number float
		width = 0,
	}
	
	return line
end

style.pane_grid = function ()
	---@class PaneGrid
	local pane_grid = {
		--- @type Highlight
		hovered_region = style.highlight(),
		--- @type Line
		picked_split = style.line(),
		--- @type Line
		hovered_split = style.line()
	}

	return pane_grid
end

style.rule = function ()
	---@class Rule
	local rule = {
		--- @type Color
		color = pallete.WHITE,
		--- @type number number > 0
		width = 0,
		--- @type Rounded
		rounded = style.rounded(0)
	}
	return rule
end

style.scroller = function ()
	---@class Scroller
	local scroller = {
		--- @type Color
		color = pallete.WHITE,
		--- @type Border
		border = style.border()
	}
	return scroller
end

style.rail = function ()
	---@class Rail
	local rail = {
		--- @type Color|nil
		background = nil,
		--- @type Border
		border = style.border(),
		--- @type Scroller
		scroller = style.scroller()
	}
	return rail
end

style.scrollable = function ()
	---@class Scrollable
	local scroll = {
		--- @type Color|nil
		gap = nil,
		--- @type Container
		container = style.container(),
		--- @type Rail
		rail = style.rail()
	}
	return scroll
end

return style