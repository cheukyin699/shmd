(ns shmd.core
  (:require [ring.adapter.jetty :use [run-jetty]]
            [compojure.core :refer :all]
            [compojure.route :as route]
            [ring.util.response :refer [response]]
            [ring.middleware.json :use [wrap-json-response]]))

(defroutes handler
  (GET "/" [] (response {:a 32 :b 42}))
  (route/not-found "not found"))

(def app
  (-> handler
      (wrap-json-response)))

(defn -main
  [& args]
  (run-jetty #'app {:port 21212
                    :join? false}))

(comment
  (defonce server (run-jetty #'app {:port 21212
                                    :join? false}))
  (.start server)
  (.stop server))
