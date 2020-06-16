FROM python:3.8
COPY ./ /srv/fun-stuff/
WORKDIR /srv/fun-stuff/
RUN pip install -r requirements.txt
EXPOSE 5001
CMD python ./src/main.py