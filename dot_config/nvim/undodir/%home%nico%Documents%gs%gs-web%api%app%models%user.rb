Vim�UnDo� ���n�[�Ahq�8�X�j�m��V�����&���   W                                  d���   " _�                     5        ����                                                                                                                                                                                                                                                                                                                                                             ds%    �   5   9   X        �   5   7   W    5��    5                      �                     �    5                      �                     �    5                     �                     �    6                     �                     �    6                    �                    �    6                    �                    �    6                     �                     �    6                    �                     �    7                     �                    �    7                     �                    5�_�                    7       ����                                                                                                                                                                                                                                                                                                                                                             ds+    �   7   9   [          �   8   9   [    �   7   9   Z    5��    7                      �                     �    7                  1   �             1       5�_�                    8       ����                                                                                                                                                                                                                                                                                                                                                             ds_    �   7   9   [      1    User.where(id: albums.pluck(:contributor_id))5��    7                                          5�_�                    7        ����                                                                                                                                                                                                                                                                                                                            7          9          V       d�͹     �   6   7            def contributors   7    User.where(id: owned_albums.pluck(:contributor_id))     end5��    6                      �      Q               5�_�                    7        ����                                                                                                                                                                                                                                                                                                                            7          7          V       d�͹    �   6   7           5��    6                      �                     5�_�                   !        ����                                                                                                                                                                                                                                                                                                                                                             d�k�    �   !   #   W    �   !   "   W    5��    !                      ~                     5�_�      	              "       ����                                                                                                                                                                                                                                                                                                                                                             d�l,     �   "   %   Y        �   "   $   X    5��    "                      �                     �    "                      �                     �    "                     �                     �    #                     �                     �    #                    �                    �    #                     �                     5�_�      
           	   $       ����                                                                                                                                                                                                                                                                                                                                                             d�l/     �   #   $            sea5��    #                      �                     5�_�   	              
   #        ����                                                                                                                                                                                                                                                                                                                                                             d�l1    �   #   %   Y    �   #   $   Y    5��    #                      �              D       5�_�   
                "       ����                                                                                                                                                                                                                                                                                                                                                             d�p�     �   !   "            include SearchHandler5��    !                      ~                     5�_�                    #        ����                                                                                                                                                                                                                                                                                                                                                             d�p�     �   "   #          C  search_scope against: :name, associated_against: { user: :email }5��    "                            D               5�_�                    #        ����                                                                                                                                                                                                                                                                                                                                                             d�p�   ! �   "   #           5��    "                                           5�_�                             ����                                                                                                                                                                                                                                                                                                                                                             d���   " �               W   # frozen_string_literal: true       class User < ApplicationRecord   *  has_paper_trail only: %i[email verified]   .  self.filter_attributes = %i[password_digest]         before_save :downcase_email   '  before_destroy :update_clip_ownership         has_many_attached :files       /  belongs_to :subscription_deal, optional: true       R  has_many :contributed_albums, foreign_key: 'contributor_id', class_name: 'Album'   [  has_many :owned_albums, foreign_key: 'owner_id', class_name: 'Album', dependent: :destroy   0  has_many :payment_methods, dependent: :destroy   .  has_many :user_sessions, dependent: :destroy   K  has_many :invitations, class_name: 'AlbumInvitation', dependent: :destroy   4  has_many :interview_templates, dependent: :destroy         has_secure_password   2  has_secure_token :verification_token, length: 24   (  has_secure_token :password_reset_token       9  validates :backup_email, email: true, allow_blank: true   A  validates :email, presence: true, uniqueness: true, email: true   !  validates :name, presence: true   \  validates :password, presence: true, length: { minimum: 8 }, if: :password_digest_changed?   �  validates :password, format: { with: /[A-Za-z]+/, message: 'must contain at least one letter' }, if: :password_digest_changed?   }  validates :password, format: { with: /[0-9]+/, message: 'must contain at least one number' }, if: :password_digest_changed?   =  validates :recording_tips, inclusion: { in: [true, false] }         include UserAdmin         def albums   =    Album.where('contributor_id = ? OR owner_id = ?', id, id)     end         def payments   7    Payment.where('album_id IN (?)', albums.pluck(:id))     end         def owner?(album)       id == album.owner_id     end         def contributor?(album)       id == album.contributor_id     end         def intervivos?   ?    subscription_deal.present? && subscription_deal.intervivos?     end       	  private         def downcase_email   %    self.email = email.downcase.strip     end         def update_clip_ownership   <    clips = Clip.includes(:album).where(created_by: user.id)           clips.each do |clip|   ,      next if clip.album.owner_id == user.id       $      owner_id = clip.album.owner_id   M      recorded_by = clip.recorded_by == user.id ? owner_id : clip.recorded_by   6      clip.update!(created_by: owner_id, recorded_by:)       end     end         public       8  def reset_password(new_password, password_reset_token)   :    update!(password: new_password, password_reset_token:)   >    UserMailer.with(user: self).password_changed.deliver_later     end       
  def user       self     end         def active_payment_method   +    payment_methods.find_by_deleted_at(nil)     end   end5�5�_�   
                 !       ����                                                                                                                                                                                                                                                                                                                                                             d�p�     �       "        5��                           j                     5�_�                            ����                                                                                                                                                                                                                                                                                                                                                             d�k�     �         W    �         W        include SearchHandler5��                          >                      5��