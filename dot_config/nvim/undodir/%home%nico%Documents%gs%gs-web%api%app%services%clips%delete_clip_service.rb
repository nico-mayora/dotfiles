Vim�UnDo� ��F2A��@����L�=C�Ӏ��?+D��   %   M    if manager_or_family_or_personal_account? || client_account_and_clip_own?   
                           c��J    _�                            ����                                                                                                                                                                                                                                                                                                                                                             c�7�    �         %          success!5��                         �                     �                         �                     5�_�                            ����                                                                                                                                                                                                                                                                                                                                                             c�N    �               %   # frozen_string_literal: true       .class Clips::DeleteClipService < ServiceObject   3  AVAILABLE_ATTRIBUTES = %i[current_user id].freeze   $  attr_reader(*AVAILABLE_ATTRIBUTES)       
  def call   H    error!(error_code: 403_00) unless clip.album.account == user_account       M    if manager_or_family_or_personal_account? || client_account_and_clip_own?         clip.remove_video         clip.save!       else          error!(error_code: 403_00)       end           success! clip     end       	  private       
  def clip       @clip ||= Clip.find(id)     end         def user_account   L    @user_account ||= current_user&.account || current_user&.client&.account     end       ,  def manager_or_family_or_personal_account?   K    current_user.manager? || current_user.family? || current_user.personal?     end       "  def client_account_and_clip_own?   ;    current_user.client? && clip.recorded_by?(current_user)     end   end5�5�_�                    
        ����                                                                                                                                                                                                                                                                                                                                                             c��D     �   	      %      M    if manager_or_family_or_personal_account? || client_account_and_clip_own?5��    	   1                 .                    �       =                 N                    5�_�                            ����                                                                                                                                                                                                                                                                                                                                                             c��G     �                L    @user_account ||= current_user&.account || current_user&.client&.account5��    !                    �                    5�_�                    "        ����                                                                                                                                                                                                                                                                                                                                                             c��H     �   !   #          "  def client_account_and_clip_own?5��    "                    #                    5�_�                     #        ����                                                                                                                                                                                                                                                                                                                                                             c��I    �   "   $          ;    current_user.client? && clip.recorded_by?(current_user)5�5��