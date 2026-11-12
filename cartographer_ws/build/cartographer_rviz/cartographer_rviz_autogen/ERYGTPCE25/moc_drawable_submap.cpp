/****************************************************************************
** Meta object code from reading C++ file 'drawable_submap.h'
**
** Created by: The Qt Meta Object Compiler version 69 (Qt 6.10.2)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include "../../../../src/cartographer_ros/cartographer_rviz/include/cartographer_rviz/drawable_submap.h"
#include <QtCore/qmetatype.h>

#include <QtCore/qtmochelpers.h>

#include <memory>


#include <QtCore/qxptype_traits.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'drawable_submap.h' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 69
#error "This file was generated using the moc from 6.10.2. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

#ifndef Q_CONSTINIT
#define Q_CONSTINIT
#endif

QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
QT_WARNING_DISABLE_GCC("-Wuseless-cast")
namespace {
struct qt_meta_tag_ZN17cartographer_rviz14DrawableSubmapE_t {};
} // unnamed namespace

template <> constexpr inline auto cartographer_rviz::DrawableSubmap::qt_create_metaobjectdata<qt_meta_tag_ZN17cartographer_rviz14DrawableSubmapE_t>()
{
    namespace QMC = QtMocConstants;
    QtMocHelpers::StringRefStorage qt_stringData {
        "cartographer_rviz::DrawableSubmap",
        "RequestSucceeded",
        "",
        "UpdateSceneNode",
        "ToggleVisibility",
        "TogglePoseMarkerVisibility"
    };

    QtMocHelpers::UintData qt_methods {
        // Signal 'RequestSucceeded'
        QtMocHelpers::SignalData<void()>(1, 2, QMC::AccessPublic, QMetaType::Void),
        // Slot 'UpdateSceneNode'
        QtMocHelpers::SlotData<void()>(3, 2, QMC::AccessPrivate, QMetaType::Void),
        // Slot 'ToggleVisibility'
        QtMocHelpers::SlotData<void()>(4, 2, QMC::AccessPrivate, QMetaType::Void),
        // Slot 'TogglePoseMarkerVisibility'
        QtMocHelpers::SlotData<void()>(5, 2, QMC::AccessPrivate, QMetaType::Void),
    };
    QtMocHelpers::UintData qt_properties {
    };
    QtMocHelpers::UintData qt_enums {
    };
    return QtMocHelpers::metaObjectData<DrawableSubmap, qt_meta_tag_ZN17cartographer_rviz14DrawableSubmapE_t>(QMC::MetaObjectFlag{}, qt_stringData,
            qt_methods, qt_properties, qt_enums);
}
Q_CONSTINIT const QMetaObject cartographer_rviz::DrawableSubmap::staticMetaObject = { {
    QMetaObject::SuperData::link<QObject::staticMetaObject>(),
    qt_staticMetaObjectStaticContent<qt_meta_tag_ZN17cartographer_rviz14DrawableSubmapE_t>.stringdata,
    qt_staticMetaObjectStaticContent<qt_meta_tag_ZN17cartographer_rviz14DrawableSubmapE_t>.data,
    qt_static_metacall,
    nullptr,
    qt_staticMetaObjectRelocatingContent<qt_meta_tag_ZN17cartographer_rviz14DrawableSubmapE_t>.metaTypes,
    nullptr
} };

void cartographer_rviz::DrawableSubmap::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    auto *_t = static_cast<DrawableSubmap *>(_o);
    if (_c == QMetaObject::InvokeMetaMethod) {
        switch (_id) {
        case 0: _t->RequestSucceeded(); break;
        case 1: _t->UpdateSceneNode(); break;
        case 2: _t->ToggleVisibility(); break;
        case 3: _t->TogglePoseMarkerVisibility(); break;
        default: ;
        }
    }
    if (_c == QMetaObject::IndexOfMethod) {
        if (QtMocHelpers::indexOfMethod<void (DrawableSubmap::*)()>(_a, &DrawableSubmap::RequestSucceeded, 0))
            return;
    }
}

const QMetaObject *cartographer_rviz::DrawableSubmap::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *cartographer_rviz::DrawableSubmap::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_staticMetaObjectStaticContent<qt_meta_tag_ZN17cartographer_rviz14DrawableSubmapE_t>.strings))
        return static_cast<void*>(this);
    return QObject::qt_metacast(_clname);
}

int cartographer_rviz::DrawableSubmap::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QObject::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 4)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 4;
    }
    if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 4)
            *reinterpret_cast<QMetaType *>(_a[0]) = QMetaType();
        _id -= 4;
    }
    return _id;
}

// SIGNAL 0
void cartographer_rviz::DrawableSubmap::RequestSucceeded()
{
    QMetaObject::activate(this, &staticMetaObject, 0, nullptr);
}
QT_WARNING_POP
