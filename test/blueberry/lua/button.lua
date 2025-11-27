local styles = require("styles")

---@return Button
---@param status Status can be: pressed, active, disable, hovered
function Style(status)
    local style = styles.button()

    if toolkit.is_active(status) then
        print("active")
    end

    style.background = nil

    return style;
end