local styles = require("styles")

---@return Appearance
function Style()
    local style = styles.appearance()
    local time = os.time()

    local r = 255 * (0.5 + 0.5 * math.sin(time))
    local g = 255 * (0.5 + 0.5 * math.sin(time + 2))
    local b = 255 * (0.5 + 0.5 * math.sin(time + 4))

    style.background = toolkit.color.rgb(r, g, b);
    style.text_color = toolkit.color.rgb(255, 255, 255);

    return style
end