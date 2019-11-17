mod service;

use router::Router;

use service::post_read_all;
use service::post_read;
use service::post_create;
use service::post_update;
use service::post_delete;

pub fn define_router(router: &mut Router) {

  router.get   ("/post",          post_read_all, "post_read_all");
  router.get   ("/post/:post_id", post_read,     "post_read");
  router.post  ("/post",          post_create,   "post_create");
  router.put   ("/post/:post_id", post_update,   "post_update");
  router.delete("/post/:post_id", post_delete,   "post_delete");

}
