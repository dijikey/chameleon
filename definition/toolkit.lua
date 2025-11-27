---@meta

-- Toolkit for "chameleon" library
---@type table
toolkit = {}
local color = {};
---@class Status
local status = nil;
---@class Color
local colorclass = {
    ---@type number In range [0, 255]
    r=0,
    ---@type number In range [0, 255]
    g=0,
    ---@type number In range [0, 255]
    b=0,
    ---@type number In range [0, 100]
    a=0
};

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
function color.rgba(r,g,b) end


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

toolkit.color = color