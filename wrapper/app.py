from django.urls import re_path
from django.shortcuts import render as django_render

DEBUG = True
SECRET_KEY = "1234"
ROOT_URLCONF = __name__
TEMPLATES = [
    {
        "BACKEND": "django.template.backends.django.DjangoTemplates",
        "DIRS": ["./wrapper/templates/"],
    },
]


def home(request):
    return django_render(request, "index.html", locals())


urlpatterns = [
    re_path(r"^$", home, name="homepage"),
]
