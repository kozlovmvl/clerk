import requests


class Response:
    status: int
    body: list or dict
    
    def __init__(self, source):
        self.status=source.status_code
        if isinstance(source, requests.models.Response):
            if source.headers['content-type'] == 'application/json':
                self.body = source.json()
            else:
                self.body = source.text
            

class BaseQueryMaker:

    def get(self, url: str, headers: dict = None, params: dict = None) -> Response:
        raise NotImplementedError

    def post(self, url: str, headers: dict = None, params: dict = None, body: dict = None) -> Response:
        raise NotImplementedError


class RequestsQueryMaker(BaseQueryMaker):

    def get(self, url: str, headers: dict = None, params: dict = None) -> Response:
        return Response(requests.get(url=url, headers=headers, params=params))

    def post(self, url: str, headers: dict = None, params: dict = None, body: dict = None) -> Response:
        return Response(requests.post(url=url, headers=headers, params=params, json=body))



request = RequestsQueryMaker()

