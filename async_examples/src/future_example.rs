trait SimpleFuture{
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T>{
    Ready(T),
    Pending,
}

// pub struct SocketRead<'a>{
//     socket: &'a Socket,
// }
//
// // 原理实例
//
// impl SimpleFuture for SocketRead<'_>{
//     type Output = Vec<u8>;
//
//     fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
//         if self.socket.has_data_to_read(){
//             Poll::Ready(self.socket.read_buf())
//         }else{
//             // 如果socket没有数据
//             // 当数据可读的时候调用wake, 再次调用poll去获取数据，
//             // 而不是不断的轮询去获取数据。
//             self.socket.set_readable_callback(wake);
//             Poll::Pending
//         }
//     }
// }

// trait Future {
//     type Output;
//     fn poll(
//         // Note the change from `&mut self` to `Pin<&mut Self>`:
//         self: Pin<&mut Self>,
//         // and the change from `wake: fn()` to `cx: &mut Context<'_>`:
//         cx: &mut Context<'_>,
//     ) -> Poll<Self::Output>;
// }
