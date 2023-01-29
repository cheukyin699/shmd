(ns shmd.core
  (:require [ring.adapter.jetty :use [run-jetty]]
            [compojure.core :refer :all]
            [compojure.route :as route]
            [ring.util.response :refer [response]]
            [ring.middleware.defaults :use [wrap-defaults api-defaults]]
            [ring.middleware.params :use [wrap-params]]
            [ring.middleware.keyword-params :use [wrap-keyword-params]]
            [ring.middleware.json :use [wrap-json-response]]))

(defroutes handler
  (GET "/" [] (response {:a 32 :b 42}))
  (GET "/thumbnail/:album" [album] (response {:album album}))
  (PATCH "/media" [& params] (response params))
  (GET "/media" [& params] (response {:data [] :params params}))
  (GET "/media/:id" [id] (response {:id id}))
  (GET "/status" [] (response {:media-in-db 0}))
  (route/not-found "not found"))

(def app
  (-> #'handler
      (wrap-keyword-params)
      (wrap-json-response)
      (wrap-defaults api-defaults)))

(defn -main
  [& args]
  (run-jetty #'app {:port 21212
                    :join? false}))

(comment
  (defonce server (run-jetty #'app {:port 21212
                                    :join? false}))
  (.start server)
  (.stop server))
