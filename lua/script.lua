--label:加工
--group:入力のリサイズ,false
--check@resize_input:有効,true
--track@resize_height:高さ,144,1080,480,1
--group:
--PARAMS
--filter
--description:VHS-like effect, powered by ntsc-rs v{{ntscrs_version}}, written in Rust / v{{mod2_version}} / https://github.com/sevenc-nanashi/ntsc-rs.auf2

local function to_json(v)
  if type(v) == "number" then
    return tostring(v)
  elseif type(v) == "string" then
    return string.format("%q", v)
  elseif type(v) == "boolean" then
    return tostring(v)
  elseif type(v) == "table" then
    local is_array = (#v > 0)
    local items = {}
    if is_array then
      for i = 1, #v do
        items[#items + 1] = to_json(v[i])
      end
      return "[" .. table.concat(items, ",") .. "]"
    else
      for k, val in pairs(v) do
        items[#items + 1] = string.format("%q:%s", k, to_json(val))
      end
      return "{" .. table.concat(items, ",") .. "}"
    end
  else
    return "null"
  end
end

local params = {
--CONVERTIONS
}

local ntsc = obj.module("ntsc-rs")
local expected_version = "{{mod2_version}}"
if ntsc.version() ~= expected_version then
  error(string.format("ntsc module version mismatch: expected %s, got %s", expected_version, ntsc.version()))
end

local resized_width, resized_height
if resize_input then
  resized_width = math.floor(obj.w * (resize_height / obj.h) + 0.5)
  resized_height = resize_height
else
  resized_width = obj.w
  resized_height = obj.h
end
local original_w, original_h = obj.w, obj.h
obj.setoption("drawtarget", "tempbuffer", resized_width, resized_height)
obj.drawpoly(
  -resized_width / 2, -resized_height / 2, 0,
   resized_width / 2, -resized_height / 2, 0,
   resized_width / 2,  resized_height / 2, 0,
  -resized_width / 2,  resized_height / 2, 0
)
local data, _, _ = obj.getpixeldata("tempbuffer")
ntsc.process(
  data,
  resized_width,
  resized_height,
  obj.frame,
  to_json(params)
)
obj.putpixeldata("tempbuffer", data, resized_width, resized_height)
obj.load("tempbuffer")
obj.setoption("drawtarget", "tempbuffer", original_w, original_h)
obj.drawpoly(
  -original_w / 2, -original_h / 2, 0,
   original_w / 2, -original_h / 2, 0,
   original_w / 2,  original_h / 2, 0,
  -original_w / 2,  original_h / 2, 0
)
obj.copybuffer("object", "tempbuffer")

-- vim: set ft=lua :
