FROM fedora:latest

RUN dnf update -y && \
    dnf install wget git meson cmake gcc gcc-c++ \
    freetype-devel fontconfig-devel libxml2-devel fribidi-devel libpng-devel libjpeg-turbo-devel libXext-devel libXrender-devel gobject-introspection-devel python3-packaging -y && \
    dnf clean all -y

RUN git clone https://gitlab.gnome.org/GNOME/glib.git --depth=1 && \
    (cd /glib && \
        meson setup builddir --prefix=/usr --buildtype release -Dtests=false && \
        meson install -C builddir) && \
    git clone https://gitlab.freedesktop.org/cairo/cairo.git --depth=1 && \
    (cd /cairo && \
        meson setup builddir --prefix=/usr --buildtype release -Dglib=enabled -Dtests=disabled && \
        meson install -C builddir) && \
    git clone https://github.com/harfbuzz/harfbuzz.git --depth=1 && \
    (cd /harfbuzz && \
        meson setup builddir --prefix=/usr --buildtype release -Dintrospection=enabled -Dtests=disabled -Ddocs=disabled && \
        meson install -C builddir) && \
    git clone https://gitlab.gnome.org/GNOME/pango.git --depth=1 && \
    (cd /pango && \
        meson setup builddir --prefix=/usr --buildtype release -Dfreetype=enabled -Dintrospection=enabled -Dxft=disabled && \
        meson install -C builddir) && \
    git clone https://gitlab.gnome.org/GNOME/gdk-pixbuf.git --depth=1 && \
    (cd /gdk-pixbuf && \
        meson setup builddir --prefix=/usr --buildtype release -Dintrospection=enabled -Dtests=false -Dman=false -Dinstalled_tests=false -Dgio_sniffing=false && \
        meson install -C builddir) && \
    git clone https://github.com/ebassi/graphene.git --depth=1 && \
    (cd /graphene && \
        meson setup builddir --prefix=/usr --buildtype release -Dintrospection=enabled -Dtests=false -Dinstalled_tests=false && \
        meson install -C builddir) && \
    rm -rf /glib /cairo /harfbuzz /pango /gdk-pixbuf /graphene

