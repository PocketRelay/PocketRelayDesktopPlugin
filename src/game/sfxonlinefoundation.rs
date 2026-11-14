use super::core::{FName, FScriptDelegate, TArray, UObject};
use std::os::raw::{c_uchar, c_ulong, c_void};

// Class SFXOnlineFoundation.SFXOnlineComponent
// 0x0028 (0x0064 - 0x003C)
#[repr(C, packed(4))]
pub struct USFXOnlineComponent {
    pub _base: UObject,
    pub event_subscriber_table: TArray<FSFXOnlineSubscriberEventType>,
    pub __on_event_delegate: FScriptDelegate,
    pub api_name: FName,
    pub online_subsystem: *mut c_void, /* USFXOnlineSubsystem */
    pub needs_state_machine: c_ulong,
}

#[repr(C, packed(4))]
pub struct FSFXOnlineSubscriberEventType {
    pub event_callback: FName,
    pub event_type: c_uchar,
}

// Class SFXOnlineFoundation.SFXOnlineComponentOrigin
// 0x012D (0x0191 - 0x0064)
#[repr(C, packed(4))]
pub struct USFXOnlineComponentOrigin {
    pub _base: USFXOnlineComponent,
    // struct FPointer                                    VfTable_IISFXOnlineComponent;                     		// 0x0064 (0x0004) [0x0000000000801002]              ( CPF_Const | CPF_Native | CPF_NoExport )
    // struct FQWord                                      mUserId;                                          		// 0x0068 (0x0008) [0x0000000000001000]              ( CPF_Native )
    // TArray< int >                                      mRetroGrantAchievementQueue;                      		// 0x0070 (0x000C) [0x0000000000001000]              ( CPF_Native )
    // struct FString                                     mPresenceString;                                  		// 0x007C (0x000C) [0x0000000000001000]              ( CPF_Native )
    // struct FString                                     mGamePresenceString;                              		// 0x0088 (0x000C) [0x0000000000001000]              ( CPF_Native )
    // struct FString                                     mSessionPresenceString;                           		// 0x0094 (0x000C) [0x0000000000001000]              ( CPF_Native )
    // struct FString                                     mDisplayName;                                     		// 0x00A0 (0x000C) [0x0000000000001000]              ( CPF_Native )
    // TArray< struct FPCFriend >                         mFriendsListCache;                                		// 0x00AC (0x000C) [0x0000000000001000]              ( CPF_Native )
    // TArray< struct FPCFriend >                         mBlockListCache;                                  		// 0x00B8 (0x000C) [0x0000000000001000]              ( CPF_Native )
    // TArray< struct FSFXOnlineEntitlementLookupInfo >   mEntitlementCache;                                		// 0x00C4 (0x000C) [0x0000000000001000]              ( CPF_Native )
    // struct FString                                     mOriginVersion;                                   		// 0x00D0 (0x000C) [0x0000000000001000]              ( CPF_Native )
    // struct FString                                     mInviteInfo;                                      		// 0x00DC (0x000C) [0x0000000000001000]              ( CPF_Native )
    // TArray< struct FScriptDelegate >                   mOriginClosedDelegates;                           		// 0x00E8 (0x000C) [0x0000000000001000]              ( CPF_Native )
    // TArray< struct FScriptDelegate >                   mOriginAuthKeyDelegates;                          		// 0x00F4 (0x000C) [0x0000000000001000]              ( CPF_Native )
    // TArray< struct FScriptDelegate >                   mWalletBalanceDelegates;                          		// 0x0100 (0x000C) [0x0000000000001000]              ( CPF_Native )
    // struct FString                                     mContentId;                                       		// 0x010C (0x000C) [0x0000000000404000]              ( CPF_Config | CPF_NeedCtorLink )
    // struct FString                                     mMultiplayerId;                                   		// 0x0118 (0x000C) [0x0000000000404000]              ( CPF_Config | CPF_NeedCtorLink )
    // struct FString                                     mCurrencyId;                                      		// 0x0124 (0x000C) [0x0000000000404000]              ( CPF_Config | CPF_NeedCtorLink )
    // struct FString                                     mOfferGroup;                                      		// 0x0130 (0x000C) [0x0000000000404000]              ( CPF_Config | CPF_NeedCtorLink )
    // struct FString                                     mAccessEntitlement;                               		// 0x013C (0x000C) [0x0000000000404000]              ( CPF_Config | CPF_NeedCtorLink )
    // struct FScriptDelegate                             __OnOriginClosedDelegate__Delegate;               		// 0x0148 (0x000C) [0x0000000000400000]              ( CPF_NeedCtorLink )
    // struct FScriptDelegate                             __OnOriginAuthKey__Delegate;                      		// 0x0154 (0x000C) [0x0000000000400000]              ( CPF_NeedCtorLink )
    // struct FScriptDelegate                             __OnWalletBalanceAvailable__Delegate;             		// 0x0160 (0x000C) [0x0000000000400000]              ( CPF_NeedCtorLink )
    // void*                                              mTitleStringRef;                                  		// 0x016C (0x0004) [0x0000000000004000]              ( CPF_Config )
    // float                                              mTimeBetweenOnlineChecks;                         		// 0x0170 (0x0004) [0x0000000000004000]              ( CPF_Config )
    // int                                                mCheckoutTimeout;                                 		// 0x0174 (0x0004) [0x0000000000004000]              ( CPF_Config )
    // int                                                mFriendQueryTimeout;                              		// 0x0178 (0x0004) [0x0000000000004000]              ( CPF_Config )
    // int                                                mEntitlementQueryTimeout;                         		// 0x017C (0x0004) [0x0000000000004000]              ( CPF_Config )
    // int                                                mQueryAchievementsTimeOut;                        		// 0x0180 (0x0004) [0x0000000000004000]              ( CPF_Config )
    // int                                                mGrantAchievementsTimeOut;                        		// 0x0184 (0x0004) [0x0000000000004000]              ( CPF_Config )
    // float                                              mTimeSinceLastOnlineCheck;                        		// 0x0188 (0x0004) [0x0000000000001000]              ( CPF_Native )
    // unsigned long                                      mDisableOrigin : 1;                               		// 0x018C (0x0004) [0x0000000000004000] [0x00000001] ( CPF_Config )
    // unsigned long                                      mServiceStarted : 1;                              		// 0x018C (0x0004) [0x0000000000001000] [0x00000002] ( CPF_Native )
    // unsigned long                                      mIsLoggedIn : 1;                                  		// 0x018C (0x0004) [0x0000000000001000] [0x00000004] ( CPF_Native )
    // unsigned long                                      mIsOriginDisabled : 1;                            		// 0x018C (0x0004) [0x0000000000001000] [0x00000008] ( CPF_Native )
    // unsigned long                                      mIsEntitlementCacheAvailable : 1;                 		// 0x018C (0x0004) [0x0000000000001000] [0x00000010] ( CPF_Native )
    // unsigned long                                      mIsPendingConnection : 1;                         		// 0x018C (0x0004) [0x0000000000001000] [0x00000020] ( CPF_Native )
    // unsigned long                                      mIsOverlayUp : 1;                                 		// 0x018C (0x0004) [0x0000000000001000] [0x00000040] ( CPF_Native )
    // unsigned long                                      mIsOverlayEnabled : 1;                            		// 0x018C (0x0004) [0x0000000000001000] [0x00000080] ( CPF_Native )
    // unsigned long                                      mOldClient : 1;                                   		// 0x018C (0x0004) [0x0000000000001000] [0x00000100] ( CPF_Native )
    // unsigned long                                      mFriendsListUpToDate : 1;                         		// 0x018C (0x0004) [0x0000000000001000] [0x00000200] ( CPF_Native )
    // unsigned long                                      mIsOnline : 1;                                    		// 0x018C (0x0004) [0x0000000000001000] [0x00000400] ( CPF_Native )
    // unsigned long                                      mPendingInviteJoin : 1;                           		// 0x018C (0x0004) [0x0000000000001000] [0x00000800] ( CPF_Native )
    // unsigned char                                      mPresenceState;
}
