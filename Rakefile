# frozen_string_literal: true
require "bundler/setup"

desc "ビルドします"
task :build => [:build_script] do
  sh "cargo build --release"
end

desc "リリースアセットを作成します"
task :release => [:build] do
  require "zip"
  require "tomlrb"

  version = Tomlrb.load_file("./Cargo.toml")["package"]["version"]
  release_md = File.read("./release.md")
  mkdir_p "./release"
  File.write("./release/README.md", release_md.gsub("{{version}}", version))

  sh "au2 release --set-version #{version}"
end

desc "Luaスクリプトをビルドして./lua/ntsc-rs.anm2に出力します"
task :build_script do
  require "yaml"
  require "json"
  require "tomlrb"

  source = File.read("./lua/script.lua")
  parameters = YAML.load_file("./parameters.yml")
  key_to_label = JSON.load_file("./lua/key_to_label.json")
  label_usage_counts = JSON.load_file("./lua/label_usage_counts.json")
  label_usage_counts.default = 0
  flat_parameters =
    parameters["parameters"].flat_map do |key, value|
      if value["type"] == "group"
        [
          [key, value.reject { |k, _v| k == "parameters" }],
          *value["parameters"].to_a,
          [nil, { "type" => "endgroup" }]
        ]
      else
        [[key, value]]
      end
    end

  (flat_parameters.size - 1).downto(1) do |i|
    if flat_parameters[i - 1][1]["type"] == "endgroup" &&
         flat_parameters[i][1]["type"] == "group"
      flat_parameters.delete_at(i - 1)
    end
  end

  parameter_definitions = []
  convertions = {}
  flat_parameters.each do |param_key, param_value|
    if param_value["type"] == "endgroup"
      parameter_definitions << "--group:"
      next
    end
    label = key_to_label[param_key]
    if not label
      label =
        param_value["label"] +
          "\u200b" * label_usage_counts[param_value["label"]]
      label_usage_counts[param_value["label"]] += 1
      key_to_label[param_key] = label
    end
    if param_value["type"] == "group"
      definition = "--group:#{label},#{param_value["opened"]}"
    else
      definition = "--#{param_value["type"]}@PARAM_#{param_key}:#{label}"
      case param_value["type"]
      when "group"
        definition += ",#{param_value["opened"]}"
      when "select"
        definition +=
          "=" +
            param_value["items"]
              .find_index { |item| item["value"] == param_value["default"] }
              .to_s
        param_value["items"].each_with_index do |item, index|
          definition += ",#{item["label"]}=#{index}"
        end
        convertions[param_key] = "({#{
          param_value["items"]
            .map
            .with_index do |item, index|
              "[#{index}] = #{item["value"].inspect}"
            end
            .join(", ")
        }})[PARAM_#{param_key}]"
      when "track"
        definition +=
          ",#{param_value["min"]},#{param_value["max"]},#{param_value["default"]},#{param_value["step"]}"
        convertions[param_key] = "PARAM_#{param_key}"
      when "check"
        definition += ",#{param_value["default"]}"
        convertions[param_key] = "PARAM_#{param_key}"
      else
        raise "Unknown parameter type: #{param_value["type"]}"
      end
    end
    parameter_definitions << definition
  end

  mod2_version = Tomlrb.load_file("./Cargo.toml")["package"]["version"]
  cargo_lock = Tomlrb.load_file("./Cargo.lock")
  ntscrs_lock = cargo_lock["package"].find { |pkg| pkg["name"] == "ntscrs" }
  ntscrs_version =
    "#{ntscrs_lock["version"]}+#{ntscrs_lock["source"].split("#").last[0..6]}"

  output =
    source
      .sub("--PARAMS", parameter_definitions.join("\n"))
      .sub(
        "--CONVERTIONS",
        convertions.map { |k, v| "[#{k.inspect}] = #{v}" }.join(",\n")
      )
      .gsub("{{mod2_version}}", mod2_version)
      .gsub("{{ntscrs_version}}", ntscrs_version)
  File.write("./lua/ntsc-rs.anm2", output)
  File.write("./lua/key_to_label.json", JSON.pretty_generate(key_to_label))
  File.write(
    "./lua/label_usage_counts.json",
    JSON.pretty_generate(label_usage_counts)
  )
  puts "Build completed: ./lua/ntsc-rs.anm2"
end

desc "aul2をビルドします"
task :build_aul2 do
  require "json"
  require "yaml"

  translations = {}
  paramters = YAML.load_file("./parameters.yml")["parameters"]
  key_to_label = JSON.load_file("./lua/key_to_label.json")
  paramters.each do |key, value|
    translations[key_to_label[key]] = value["english_label"]
    if value["type"] == "select"
      value["items"].each do |item|
        translations[item["label"]] = item["english_label"]
      end
    end
    if value["type"] == "group"
      value["parameters"].each do |subkey, subvalue|
        translations[key_to_label[subkey]] = subvalue["english_label"]
        if subvalue["type"] == "select"
          subvalue["items"].each do |item|
            translations[item["label"]] = item["english_label"]
          end
        end
      end
    end
  end

  File.open("./target/English.ntsc-rs.aul2", "w") do |file|
    file.puts <<~EOS
    ; parameters.ymlとRakefileから自動生成。
    ; 手動で編集しないこと。
    [ntsc-rs]

    入力のリサイズ=Resize Input
    有効=Enabled
    高さ=Height
    EOS
    translations.each do |jp, en|
      file.puts "#{jp}=#{en}"
    end
  end
end
