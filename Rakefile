# frozen_string_literal: true
require "bundler/setup"
require "fileutils"

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

  dest_dir = "./test_environment/data/Plugin"
  target = "debug"
  FileUtils.mkdir_p(dest_dir) unless Dir.exist?(dest_dir)
  ln_s "#{__dir__}/target/#{target}/ntscrs_auf2.dll", File.join(dest_dir, "ntsc_rs.auf2"), force: true
end
