Vim�UnDo� ��/+�Da������޷Fo��"��f�2J��   �                                   d�O    _�                             ����                                                                                                                                                                                                                                                                                                                            �          �          V       d�N     �               �   # frozen_string_literal: true       /class AttachRemoteRecordingJob < ApplicationJob     queue_as :default         @audio_file_present = true       3  class AudioNotFoundException < StandardError; end       F  # In case an exception is raised (such as AWS::NotFound), DelayedJob   E  # retries the job up to 25 times, with an exponential backoff after      # which, the job is discarded.     def perform(args)       @audio_file_present = true   *    recording_path = args[:recording_path]       clip = args[:clip]   "    return unless clip.processing?           @clip_id = clip.id   g    Rails.logger.info "Processing videocall for clip #{@clip_id} with recording path #{recording_path}"       <    webm_file_path = process_webm_file(clip, recording_path)   '    File.open(webm_file_path) do |file|   $      attach_and_process(file, clip)       end   >    File.delete(webm_file_path) if File.exist?(webm_file_path)     end       	  private       -  def process_webm_file(clip, recording_path)   >    webm_file = fetch_webm_file(clip.album_id, recording_path)       File.path(webm_file)     end       *  def fetch_webm_file(album_id, file_path)   5    user_contributor = Contributor.find_by(album_id:)   -    split_file_path = file_path.split('_', 2)   K    s3_files = filter_s3_list_objects(split_file_path[0], user_contributor)   '    files = download_s3_files(s3_files)   g    audio_contributor, video_file, audio_manager = fetch_audio_and_video_files(files, user_contributor)   T    webm_output_file = convert_to_webm(audio_contributor, video_file, audio_manager)           files.each_value do |file|   ,      File.delete(file) if File.exist?(file)       end           webm_output_file     end       C  def convert_to_webm(audio_contributor, video_file, audio_manager)   M    audio_output_file = process_audio_files(audio_contributor, audio_manager)   6    webm_output_file = process_video_files(video_file)   T    output_file_raw = process_video_audio_files(audio_output_file, webm_output_file)   T    output_file = offset_audio(audio_output_file, webm_output_file, output_file_raw)           output_file     ensure   D    File.delete(audio_output_file) if File.exist?(audio_output_file)   B    File.delete(webm_output_file) if File.exist?(webm_output_file)   W    File.delete(output_file_raw) if File.exist?(output_file_raw) && @audio_file_present     end       D  def process_video_audio_files(audio_output_file, output_webm_file)   C    output_file = create_temp_file_route("output-#{@clip_id}.webm")       )    command = if audio_output_file.empty?   +                @audio_file_present = false   W                message = "Video processed without audio file for clip_id: #{@clip_id}"   ?                exception = AudioNotFoundException.new(message)   9                SentryHelper.capture_exception(exception)   K                "ffmpeg -i #{output_webm_file} -c:v copy -y #{output_file}"                 else   I                inputs = "-i #{output_webm_file} -i #{audio_output_file}"   j                "ffmpeg #{inputs} -map 0:v:0 -map 1 -c:v copy -threads 2 -preset medium -y #{output_file}"                 end       #    process_ffmpeg_command(command)       output_file     end       X  # The audio processed by process_video_audio_files is slightly shorter than the video.   V  # This function fixes that delay by offseting the audio by the difference in length.   5  def offset_audio(audio_file, video_file, webm_file)   /    return webm_file unless @audio_file_present       �    audio_length = process_ffmpeg_command("ffprobe -i #{audio_file} -show_entries format=duration -v quiet -of csv=\"p=0\"").to_f   �    video_length = process_ffmpeg_command("ffprobe -i #{video_file} -show_entries format=duration -v quiet -of csv=\"p=0\"").to_f   (    offset = audio_length - video_length       Q    output_file = create_temp_file_route("output-delayed-audio-#{@clip_id}.webm")   w    command = "ffmpeg -i #{webm_file} -itsoffset #{offset} -i #{webm_file} -map 1:v -map 0:a -c copy #{output_file} -y"   #    process_ffmpeg_command(command)       output_file     end       %  def process_video_files(video_file)   M    output_webm_file = create_temp_file_route("output-webm-#{@clip_id}.webm")   u    command = "ffmpeg -allowed_extensions ALL -i #{video_file} -vcodec copy -copytb 1 -copyts #{output_webm_file} -y"   #    process_ffmpeg_command(command)       output_webm_file     end       ;  def process_audio_files(audio_contributor, audio_manager)   A    return '' if audio_contributor.empty? && audio_manager.empty?           inputs = ''.dup   N    inputs += "-i #{audio_contributor} " unless audio_contributor.strip.empty?   E    inputs += "-i #{audio_manager}" unless audio_manager.strip.empty?       Z    both_audios_present = audio_contributor.strip.present? && audio_manager.strip.present?   `    filter_complex = both_audios_present ? '-filter_complex amix=inputs=2:duration=longest' : ''       Q    audio_output_file = create_temp_file_route("original_audio-#{@clip_id}.webm")   o    command = "ffmpeg -allowed_extensions ALL #{inputs} #{filter_complex} -c:a libopus -y #{audio_output_file}"   #    process_ffmpeg_command(command)           audio_output_file     end       %  def process_ffmpeg_command(command)   ?    Open3.popen3(command) do |_stdin, stdout, stderr, wait_thr|         output = stdout.read         puts output         puts stderr.read   l      raise "Command failed with exit status #{wait_thr.value.exitstatus}" if wait_thr.value.exitstatus != 0             return output       end     end       &  def create_temp_file_route(filename)   $    Rails.root.join('tmp', filename)     end       !  def download_s3_files(s3_files)       files = {}       s3_files.each do |s3_file|   (      remove_prefix = s3_file.split('/')   $      file_name = remove_prefix.last   B      tempfile = File.open(create_temp_file_route(file_name), 'w')   x      client.get_object({ bucket: Rails.application.config.aws_agora_bucket_name, key: s3_file }, target: tempfile.path)   *      files["tmp/#{file_name}"] = tempfile       end       	    files     end       :  def fetch_audio_and_video_files(files, user_contributor)       audio_contributor = ''       video_file = ''       audio_manager = ''       files.each_key do |k|   d      audio_contributor = k if k.include?('audio.m3u8') && k.include?(user_contributor.user_id.to_s)   a      audio_manager = k if k.include?('audio.m3u8') && !k.include?(user_contributor.user_id.to_s)   /      video_file = k if k.include? 'video.m3u8'       end       2    [audio_contributor, video_file, audio_manager]     end       6  def filter_s3_list_objects(prefix, user_contributor)       s3_files = []   `    files = client.list_objects(bucket: Rails.application.config.aws_agora_bucket_name, prefix:)   !    files.contents.each do |file|   l      s3_files << file.key if file.key.include?(user_contributor.user_id.to_s) || file.key.include?('audio')       end           s3_files     end         def client   $    @client ||= Aws::S3::Client.new(   2      region: Rails.application.config.aws_region,   =      access_key_id: Rails.application.config.aws_access_key,   @      secret_access_key: Rails.application.config.aws_secret_key       )     end         def bucket   S    @bucket ||= Aws::S3::Bucket.new(Rails.application.config.aws_agora_bucket_name)     end         def initial_time   "    @initial_time ||= Time.current     end       )  def download_to_tempfile(key, filename)   .    tempfile = Tempfile.new("#{filename}.mp4")   n    client.get_object({ bucket: Rails.application.config.aws_agora_bucket_name, key: }, target: tempfile.path)       tempfile     end       $  def attach_and_process(file, clip)   C    clip.attach_video(io: file, filename: File.basename(file.path))       (    movie = FFMPEG::Movie.new(file.path)   9    clip.video.blob.metadata[:recorded_at] = initial_time   8    clip.video.blob.metadata[:duration] = movie.duration       #    generate_thumbnail(movie, clip)       clip.save!     end       %  def generate_thumbnail(movie, clip)   ,    tempfile = Tempfile.new('thumbnail.jpg')   1    movie.screenshot(tempfile.path, seek_time: 2)       B    clip.thumbnail.attach(io: tempfile, filename: 'thumbnail.jpg')     ensure       tempfile&.close!     end   end5�5�_�                     �        ����                                                                                                                                                                                                                                                                                                                            �           �           V        dF�    �   �   �   �      )  def download_to_tempfile(key, filename)   .    tempfile = tempfile.new("#{filename}.mp4")   n    client.get_object({ bucket: rails.application.config.aws_agora_bucket_name, key: }, target: tempfile.path)       tempfile     end5��    �                    -                    �    �                     m                    5��