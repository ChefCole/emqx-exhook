use tracing::{Level,info};
use tracing_subscriber::FmtSubscriber;
use exhook::hook_provider_server::HookProvider;
use exhook::hook_provider_server::HookProviderServer;
use core::pin::Pin;
//use std::pin::Pin;
use core::future::Future;
pub mod exhook{
    tonic::include_proto!("emqx.exhook.v2");
}
use tonic::transport::Server;

#[derive(Debug, Default)]
pub struct MyHookProvider{}


impl HookProvider for MyHookProvider{

    // 定义 HookProvider 如何被加载，返回需要挂载的钩子列表。仅在该列表中的钩子会被回调到 HookProivder 服务。
    fn on_provider_loaded< 'life0, 'async_trait>(& 'life0 self,_request:tonic::Request<exhook::ProviderLoadedRequest> ,) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<tonic::Response<exhook::LoadedResponse> ,tonic::Status> > + core::marker::Send+ 'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        let box_res = Box::new(async {
            info!("加载");
            //创建需要hook的方法
            let rep = exhook::LoadedResponse{
                hooks:vec![
                    exhook::HookSpec{
                        name:"client.connect".to_string(),
                        topics:vec![]
                    },
                    exhook::HookSpec{
                        name:"message.publish".to_string(),
                        topics:vec![]
                    }
                ]
            };
            Ok(tonic::Response::new(rep))
        });
        unsafe{
            Pin::new_unchecked(box_res)
        }
    }

    // 通知用户该 HookProvier 已经从 emqx 中卸载。
    fn on_provider_unloaded< 'life0, 'async_trait>(& 'life0 self,_request:tonic::Request<exhook::ProviderUnloadedRequest> ,) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<tonic::Response<exhook::EmptySuccess> ,tonic::Status> > + core::marker::Send+ 'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        info!("卸载");
        let box_res = Box::new(async {
            let rep = exhook::EmptySuccess{
            };
            Ok(tonic::Response::new(rep))
        });
        unsafe{
            Pin::new_unchecked(box_res)
        }
    }
    
    // 处理连接报文 服务端收到客户端的连接报文时
    fn on_client_connect< 'life0, 'async_trait>(& 'life0 self,_request:tonic::Request<exhook::ClientConnectRequest> ,) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<tonic::Response<exhook::EmptySuccess> ,tonic::Status> > + core::marker::Send+ 'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        let box_res = Box::new(async {
            info!("客户端连接");
            let rep = exhook::EmptySuccess{
            };
            Ok(tonic::Response::new(rep))
        });
        unsafe{
            Pin::new_unchecked(box_res)
        }
    }

    // 下发连接应答	服务端准备下发连接应答报文时
    fn on_client_connack< 'life0, 'async_trait>(& 'life0 self,_request:tonic::Request<exhook::ClientConnackRequest> ,) ->  Pin<Box<dyn Future<Output = Result<tonic::Response<exhook::EmptySuccess> ,tonic::Status> > + core::marker::Send+ 'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        let box_res = Box::new(async {
            let rep = exhook::EmptySuccess{
            };
            Ok(tonic::Response::new(rep))
        });
        unsafe{
            Pin::new_unchecked(box_res)
        }
    }

    // 成功接入	客户端认证完成并成功接入系统后
    fn on_client_connected< 'life0, 'async_trait>(& 'life0 self,_request:tonic::Request<exhook::ClientConnectedRequest> ,) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<tonic::Response<exhook::EmptySuccess> ,tonic::Status> > + core::marker::Send+ 'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        let box_res = Box::new(async {
            let rep = exhook::EmptySuccess{
            };
            Ok(tonic::Response::new(rep))
        });
        unsafe{
            Pin::new_unchecked(box_res)
        }
    }

    // 连接断开	客户端连接层在准备关闭时
    fn on_client_disconnected< 'life0, 'async_trait>(& 'life0 self,_request:tonic::Request<exhook::ClientDisconnectedRequest> ,) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<tonic::Response<exhook::EmptySuccess> ,tonic::Status> > + core::marker::Send+ 'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        let box_res = Box::new(async {
            let rep = exhook::EmptySuccess{
            };
            Ok(tonic::Response::new(rep))
        });
        unsafe{
            Pin::new_unchecked(box_res)
        }
    }

    // 连接认证	执行完 client.connect 后
    fn on_client_authenticate< 'life0, 'async_trait>(& 'life0 self,_request:tonic::Request<exhook::ClientAuthenticateRequest> ,) ->  Pin<Box<dyn Future<Output = Result<tonic::Response<exhook::ValuedResponse> ,tonic::Status> > + core::marker::Send+ 'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        let box_res = Box::new(async {
            let rep = exhook::ValuedResponse{
                r#type:0,
                value:None
            };
            Ok(tonic::Response::new(rep))
        });
        unsafe{
            Pin::new_unchecked(box_res)
        }
    }

    // 发布订阅鉴权	执行 发布/订阅 操作前
    fn on_client_authorize< 'life0, 'async_trait>(& 'life0 self,request:tonic::Request<exhook::ClientAuthorizeRequest> ,) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<tonic::Response<exhook::ValuedResponse> ,tonic::Status> > + core::marker::Send+ 'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        let box_res = Box::new(async {
            let rep = exhook::ValuedResponse{
                r#type:0,
                value:None
            };
            Ok(tonic::Response::new(rep))
        });
        unsafe{
            Pin::new_unchecked(box_res)
        }
    }
    
    // 订阅主题	收到订阅报文后，执行 client.authorize 鉴权前
    fn on_client_subscribe< 'life0, 'async_trait>(& 'life0 self,_request:tonic::Request<exhook::ClientSubscribeRequest> ,) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<tonic::Response<exhook::EmptySuccess> ,tonic::Status> > + core::marker::Send+ 'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        let box_res = Box::new(async {
            let rep = exhook::EmptySuccess{
            };
            Ok(tonic::Response::new(rep))
        });
        unsafe{
            Pin::new_unchecked(box_res)
        }
    }

    //取消订阅	收到取消订阅报文后
    fn on_client_unsubscribe< 'life0, 'async_trait>(& 'life0 self,_request:tonic::Request<exhook::ClientUnsubscribeRequest> ,) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<tonic::Response<exhook::EmptySuccess> ,tonic::Status> > + core::marker::Send+ 'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        let box_res = Box::new(async {
            let rep = exhook::EmptySuccess{
            };
            Ok(tonic::Response::new(rep))
        });
        unsafe{
            Pin::new_unchecked(box_res)
        }
    }

    // 会话创建	client.connected 执行完成，且创建新的会话后
    fn on_session_created< 'life0, 'async_trait>(& 'life0 self,_request:tonic::Request<exhook::SessionCreatedRequest> ,) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<tonic::Response<exhook::EmptySuccess> ,tonic::Status> > + core::marker::Send+ 'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        let box_res = Box::new(async {
            let rep = exhook::EmptySuccess{
            };
            Ok(tonic::Response::new(rep))
        });
        unsafe{
            Pin::new_unchecked(box_res)
        }
    }

    //会话订阅主题	完成订阅操作后
    fn on_session_subscribed< 'life0, 'async_trait>(& 'life0 self,_request:tonic::Request<exhook::SessionSubscribedRequest> ,) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<tonic::Response<exhook::EmptySuccess> ,tonic::Status> > + core::marker::Send+ 'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        let box_res = Box::new(async {
            let rep = exhook::EmptySuccess{
            };
            Ok(tonic::Response::new(rep))
        });
        unsafe{
            Pin::new_unchecked(box_res)
        }
    }

    // 会话取消订阅	完成取消订阅操作后
    fn on_session_unsubscribed< 'life0, 'async_trait>(& 'life0 self,_request:tonic::Request<exhook::SessionUnsubscribedRequest> ,) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<tonic::Response<exhook::EmptySuccess> ,tonic::Status> > + core::marker::Send+ 'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        let box_res = Box::new(async {
            let rep = exhook::EmptySuccess{
            };
            Ok(tonic::Response::new(rep))
        });
        unsafe{
            Pin::new_unchecked(box_res)
        }
    }

    // 会话恢复	client.connected 执行完成，且成功恢复旧的会话信息后
    fn on_session_resumed< 'life0, 'async_trait>(& 'life0 self,_request:tonic::Request<exhook::SessionResumedRequest> ,) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<tonic::Response<exhook::EmptySuccess> ,tonic::Status> > + core::marker::Send+ 'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        let box_res = Box::new(async {
            let rep = exhook::EmptySuccess{
            };
            Ok(tonic::Response::new(rep))
        });
        unsafe{
            Pin::new_unchecked(box_res)
        }
    }

    // 会话被移除	会话由于被移除而终止后
    fn on_session_discarded< 'life0, 'async_trait>(& 'life0 self,_request:tonic::Request<exhook::SessionDiscardedRequest> ,) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<tonic::Response<exhook::EmptySuccess> ,tonic::Status> > + core::marker::Send+ 'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        let box_res = Box::new(async {
            let rep = exhook::EmptySuccess{
            };
            Ok(tonic::Response::new(rep))
        });
        unsafe{
            Pin::new_unchecked(box_res)
        }
    }

    // 会话被接管	会话由于被接管而终止后
    fn on_session_takenover< 'life0, 'async_trait>(& 'life0 self,request:tonic::Request<exhook::SessionTakenoverRequest> ,) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<tonic::Response<exhook::EmptySuccess> ,tonic::Status> > + core::marker::Send+ 'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        let box_res = Box::new(async {
            let rep = exhook::EmptySuccess{
            };
            Ok(tonic::Response::new(rep))
        });
        unsafe{
            Pin::new_unchecked(box_res)
        }
    }

    // 会话终止	会话由于其他原因被终止后
    fn on_session_terminated< 'life0, 'async_trait>(& 'life0 self,_request:tonic::Request<exhook::SessionTerminatedRequest> ,) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<tonic::Response<exhook::EmptySuccess> ,tonic::Status> > + core::marker::Send+ 'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        let box_res = Box::new(async {
            let rep = exhook::EmptySuccess{
            };
            Ok(tonic::Response::new(rep))
        });
        unsafe{
            Pin::new_unchecked(box_res)
        }
    }

    // 消息发布	服务端在发布（路由）消息前
    fn on_message_publish< 'life0, 'async_trait>(& 'life0 self,request:tonic::Request<exhook::MessagePublishRequest> ,) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<tonic::Response<exhook::ValuedResponse> ,tonic::Status> > + core::marker::Send+ 'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        let box_res = Box::new(async move{
            //获取发送的消息对象
            let message = request.get_ref();
            info!("发送消息:{:?}",message);
            let mut rep = exhook::ValuedResponse{
                r#type:0,
                value:None
            };

            if message.message.is_some() {
                let val = exhook::valued_response::Value::Message(message.message.clone().unwrap());
                rep.value = Some(val);
            }
            
            Ok(tonic::Response::new(rep))
        });
        unsafe{
            Pin::new_unchecked(box_res)
        }
    }

    // 消息投递	消息准备投递到客户端前
    fn on_message_delivered< 'life0, 'async_trait>(& 'life0 self,_request:tonic::Request<exhook::MessageDeliveredRequest> ,) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<tonic::Response<exhook::EmptySuccess> ,tonic::Status> > + core::marker::Send+ 'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        let box_res = Box::new(async {
            let rep = exhook::EmptySuccess{
            };
            Ok(tonic::Response::new(rep))
        });
        unsafe{
            Pin::new_unchecked(box_res)
        }
    }

    // 消息回执	服务端在收到客户端发回的消息 ACK 后
    fn on_message_acked< 'life0, 'async_trait>(& 'life0 self,_request:tonic::Request<exhook::MessageAckedRequest> ,) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<tonic::Response<exhook::EmptySuccess> ,tonic::Status> > + core::marker::Send+ 'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        let box_res = Box::new(async {
            let rep = exhook::EmptySuccess{
            };
            Ok(tonic::Response::new(rep))
        });
        unsafe{
            Pin::new_unchecked(box_res)
        }
    }
    
    //消息丢弃	发布出的消息被丢弃后
    fn on_message_dropped< 'life0, 'async_trait>(& 'life0 self,_request:tonic::Request<exhook::MessageDroppedRequest> ,) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<tonic::Response<exhook::EmptySuccess> ,tonic::Status> > + core::marker::Send+ 'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        let box_res = Box::new(async {
            let rep = exhook::EmptySuccess{
            };
            Ok(tonic::Response::new(rep))
        });
        unsafe{
            Pin::new_unchecked(box_res)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let addr = "0.0.0.0:9000".parse()?;
    println!("{:?}",addr);
    let hook = MyHookProvider::default();

    let _ = Server::builder()
        .add_service(HookProviderServer::new(hook))
        .serve(addr)
        .await?;

    Ok(())
}
