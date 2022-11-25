#lang racket/base

(require net/url
         (prefix-in files: web-server/dispatchers/dispatch-files)
         (prefix-in filter: web-server/dispatchers/dispatch-filter)
         (prefix-in sequencer: web-server/dispatchers/dispatch-sequencer)
         web-server/dispatchers/filesystem-map
         web-server/http/json
         web-server/http/request-structs
         web-server/servlet-dispatch
         web-server/web-server)

(require (prefix-in config: "config.rkt")
         (prefix-in routes: "routes.rkt"))

(define url->path/static
  (make-url->path config:music-path))

(define static-dispatcher
  (files:make
    #:url->path (λ (u) (url->path/static
                          (struct-copy url u [path (cdr (url-path u))])))))

(define (not-found req)
  (response/jsexpr
    (make-hash `((success . #f)
                 (error . ,(format "Couldn't access path '~a'; not found" (url->string (request-uri req))))))))

(define stop
  (serve
    #:dispatch (sequencer:make
                 (filter:make #rx"^/fs/" static-dispatcher)
                 (dispatch/servlet routes:all-routes)
                 (dispatch/servlet not-found))
    #:listen-ip config:server-ip
    #:port config:server-port))

(with-handlers ([exn:break? (λ (e) stop)])
  (sync/enable-break never-evt))
