#lang racket/base

(require web-server/http/request-structs
         web-server/http/json
         web-server/dispatch)

(provide all-routes
         make-uri)

(define (homepage req)
  (response/jsexpr
    #hash((success . #t)
          (data . (1 2 3 4)))))

(define (blog req)
  (response/jsexpr
    #hash((success . #f)
          (error . "you are bad at this"))))

(define-values (all-routes make-uri)
  (dispatch-rules
    [("") homepage]
    [("blog") blog]))

