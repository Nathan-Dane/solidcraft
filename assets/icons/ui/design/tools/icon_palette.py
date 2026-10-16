"""Shared semantic colour policy for repository icon tools; values live in palette.toml."""
import re

VISIONS = ('off', 'red-green', 'blue-yellow')


def validation_errors(palette, ids):
    errors, assigned = [], set()
    accents = palette.get('accents', {})
    for accent, icons in palette.get('icon-accents', {}).items():
        if accent not in accents:
            errors.append('unknown semantic accent ' + accent)
        for icon in icons:
            if icon not in ids or icon in assigned:
                errors.append('unknown or repeated semantic icon ' + icon)
            assigned.add(icon)
    groups = [('', accents)] + list(palette.get('variants', {}).items())
    for vision, group in groups:
        if vision and vision not in VISIONS:
            errors.append('unknown colour-vision group ' + vision)
        for accent, roles in group.items():
            if accent not in accents:
                errors.append('variant has unknown accent ' + accent)
            for role, value in roles.items():
                if role not in palette['roles']:
                    errors.append('unknown colour role ' + role)
                if 'use' in value:
                    value = roles.get(value['use'], {})
                for theme in ('light', 'dark'):
                    if not re.fullmatch(r'#[0-9a-fA-F]{6}', value.get(theme, '')):
                        errors.append(f'{vision}/{accent}/{role}: invalid {theme} hex or role link')
    return errors


def default_accent(palette, icon):
    return next((accent for accent, ids in palette.get('icon-accents', {}).items() if icon in ids), '')


def resolve(palette, aliases, icon):
    """Explicit alias accents win; defaults belong to an id, not its shared master."""
    alias = aliases.get(icon, icon)
    if isinstance(alias, dict):
        return alias['master'], alias.get('accent') or default_accent(palette, icon)
    return alias, default_accent(palette, icon)


def role_values(palette, theme, accent='', vision='off'):
    values = {role: value[theme] for role, value in palette['roles'].items()}
    overrides = palette.get('accents', {}).get(accent, {})
    for role, value in overrides.items():
        if 'use' in value:
            value = overrides[value['use']]
        values[role] = value[theme]
    for role, value in palette.get('variants', {}).get(vision, {}).get(accent, {}).items():
        values[role] = value[theme]
    return values


def themed(svg, palette, theme, accent='', vision='off'):
    values = role_values(palette, theme, accent, vision)
    paints = {value.get('source', value['light']).lower(): values[role] for role, value in palette['roles'].items() if role != 'line'}
    svg = re.sub(r'#[0-9a-fA-F]{6}', lambda match: paints.get(match[0].lower(), match[0]), svg)
    return svg.replace('currentColor', values['line'])


def review_css(palette):
    """Review controls follow the same shared colours; artwork is rendered by themed()."""
    rules = []
    for dark in (False, True):
        theme = 'dark' if dark else 'light'
        for vision in VISIONS:
            selector = 'body' + ('.dark' if dark else ':not(.dark)') + '[data-vision="' + vision + '"]'
            primary = lambda accent='': role_values(palette, theme, accent, vision).get('active-3', palette['roles']['line'][theme])
            values = {'accent': primary(),
                      'accent-ink': palette['roles'].get('gap', palette['roles']['line'])[theme],
                      'success': primary('sketch'),
                      'warning': primary('mesh'),
                      'error': primary('constraint')}
            rules.append(selector + '{' + ''.join('--review-' + key + ':' + value + ';' for key, value in values.items()) + '}')
            for accent in palette.get('accents', {}):
                rules.append(selector + ' .card[data-accent="' + accent + '"]{--icon-accent:' + primary(accent) + '}')
    return '\n'.join(rules)
