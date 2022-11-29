#lang racket/base

(require web-server/http/request-structs
         web-server/http/json
         web-server/dispatch
         net/url-structs
         racket/bool
         racket/dict)

(require (prefix-in db: "database.rkt"))

(provide all-routes
         make-uri)

(define (list-media req)
  (let* ([query (url-query (request-uri req))]
         [artist (dict-ref query 'artist "")]
         [album (dict-ref query 'album "")]
         [keyword (dict-ref query 'keyword "")]
         [limit (dict-ref query 'limit 10)]
         [offset (dict-ref query 'offset 0)])
    (response/jsexpr
      #hash((success . #t)
            (data . ,(db:get-media #:artist artist
                                   #:album album
                                   #:keyword keyword
                                   #:limit limit
                                   #:offset offset))))))

(define (count-media req)
  (let* ([query (url-query (request-uri req))]
         [artist (dict-ref query 'artist "")]
         [album (dict-ref query 'album "")]
         [keyword (dict-ref query 'keyword "")])
    (response/jsexpr
      #hash((success . #t)
            (data . ,(db:count-media #:artist artist
                                     #:album album
                                     #:keyword keyword))))))

(define (get-media req mediaid)
  (let ([result (db:get-media-by-id mediaid)])
    (if (false? result)
      (response/jsexpr
        #hash((success . #f)))
      (response/jsexpr
        #hash((success . #t)
              (data . ,result))))))

(define (get-thumbnail req album)
  #f)

(define (get-status req)
  (response/jsexpr
    #hash((success . #t)
          (count . (db:count-media)))))

(define-values (all-routes make-uri)
  (dispatch-rules
    [("status") get-status]
    [("media" (integer-arg)) get-media]
    [("media") list-media]
    [("thumbnail" (string-arg)) get-thumbnail]))

