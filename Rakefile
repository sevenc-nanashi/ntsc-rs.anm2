# frozen_string_literal: true
require "bundler/setup"

desc "ビルドします"
task :build => [:build_script] do
  sh "cargo build --release"
end

desc "リリースアセットを作成します"
task :release => [:build] do
  require "zip"

  rm_rf "release" if Dir.exist?("release")
  mkdir "release"
  cp "./release.md", "./release/README.md"
  Zip::File.open("./release/ntsc-rs.au2pkg.zip", create: true) do |zipfile|
    zipfile.mkdir("Script")
    zipfile.add("Script/ntsc-rs.anm2", "./lua/ntsc-rs.anm2")
    zipfile.add("Script/ntsc-rs.mod2", "./target/release/ntscrs_anm2.dll")
  end
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
  label_usage_counts["入力リサイズ"] += 1
  label_usage_counts["有効"] += 1
  label_usage_counts["高さ"] += 1
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

desc "./test_environment下にAviUtl2をセットアップし、debugビルドへのシンボリックリンクを作成します"
task :debug_setup do |task, args|
  require "zip"

  unless File.exist?("./test_environment/aviutl2.exe")
    zip_path = "./test_environment/aviutl2_latest.zip"
    mkdir_p("./test_environment") unless Dir.exist?("./test_environment")
    File.open(zip_path, "wb") do |file|
      require "open-uri"
      URI.open(
        "https://api.aviutl2.jp/download?version=latest&type=zip"
      ) { |uri| file.write(uri.read) }
    end
    Zip::File.open(zip_path) do |zip_file|
      zip_file.each do |entry|
        dest_path = File.join("./test_environment", entry.name)
        unless Dir.exist?(File.dirname(dest_path))
          mkdir_p(File.dirname(dest_path))
        end
        rm_rf(dest_path) if File.exist?(dest_path)
        zip_file.extract(entry, dest_path)
      end
    end
    rm(zip_path)
  end

  dest_dir = "./test_environment/data/Script"
  target = "debug"
  FileUtils.mkdir_p(dest_dir) unless Dir.exist?(dest_dir)
  ln_s "#{__dir__}/target/#{target}/ntscrs_anm2.dll",
       File.join(dest_dir, "ntsc-rs.mod2"),
       force: true
  ln_s "#{__dir__}/lua/ntsc-rs.anm2", File.join(dest_dir, "ntsc-rs.anm2"), force: true
end
