use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Deserialize, Serialize, Clone)]
pub struct TextureDesc {
    pub uv: [f32; 4],
    pub normals_uv: Option<[f32; 4]>,
    pub offset: [f32; 2],
    pub rel_size: [f32; 2],
}
#[derive(Deserialize, Serialize)]
pub struct Test{
  tex1: TextureDesc,
  tex2: TextureDesc,
  tex3: TextureDesc,
  tex4: TextureDesc,
  tex5: TextureDesc,
  tex6: TextureDesc,
  tex7: TextureDesc,
  tex8: TextureDesc,
  tex9: TextureDesc,
  tex10: TextureDesc,
  tex11: TextureDesc,
  tex12: TextureDesc,
  tex13: TextureDesc,
  tex14: TextureDesc,
  tex15: TextureDesc,
  tex16: TextureDesc,
  tex17: TextureDesc,
  tex18: TextureDesc,
  tex19: TextureDesc,
  tex20: TextureDesc,
  tex21: TextureDesc,
  tex22: TextureDesc,
  tex23: TextureDesc,
  tex24: TextureDesc,
  tex25: TextureDesc,
  tex26: TextureDesc,
  tex27: TextureDesc,
  tex28: TextureDesc,
  tex29: TextureDesc,
  tex30: TextureDesc,
  tex31: TextureDesc,
  tex32: TextureDesc,
  tex33: TextureDesc,
  tex34: TextureDesc,
  tex35: TextureDesc,
  tex36: TextureDesc,
  tex37: TextureDesc,
  tex38: TextureDesc,
  tex39: TextureDesc,
  tex40: TextureDesc,
  tex41: TextureDesc,
  tex42: TextureDesc,
  tex43: TextureDesc,
  tex44: TextureDesc,
  tex45: TextureDesc,
  tex46: TextureDesc,
  tex47: TextureDesc,
  tex48: TextureDesc,
  tex49: TextureDesc,
  tex50: TextureDesc,
  tex51: TextureDesc,
  tex52: TextureDesc,
  tex53: TextureDesc,
  tex54: TextureDesc,
  tex55: TextureDesc,
  tex56: TextureDesc,
  tex57: TextureDesc,
  tex58: TextureDesc,
  tex59: TextureDesc,
  tex60: TextureDesc,
  tex61: TextureDesc,
  tex62: TextureDesc,
  tex63: TextureDesc,
  tex64: TextureDesc,
  tex65: TextureDesc,
  tex66: TextureDesc,
  tex67: TextureDesc,
  tex68: TextureDesc,
  tex69: TextureDesc,
  tex70: TextureDesc,
  tex71: TextureDesc,
  tex72: TextureDesc,
  tex73: TextureDesc,
  tex74: TextureDesc,
  tex75: TextureDesc,
  tex76: TextureDesc,
  tex77: TextureDesc,
  tex78: TextureDesc,
  tex79: TextureDesc,
  tex80: TextureDesc,
  tex81: TextureDesc,
  tex82: TextureDesc,
  tex83: TextureDesc,
  tex84: TextureDesc,
  tex85: TextureDesc,
  tex86: TextureDesc,
  tex87: TextureDesc,
  tex88: TextureDesc,
  tex89: TextureDesc,
  tex90: TextureDesc,
  tex91: TextureDesc,
  tex92: TextureDesc,
  tex93: TextureDesc,
  tex94: TextureDesc,
  tex95: TextureDesc,
  tex96: TextureDesc,
  tex97: TextureDesc,
  tex98: TextureDesc,
  tex99: TextureDesc,
  tex100: TextureDesc,
  tex101: TextureDesc,
  tex102: TextureDesc,
  tex103: TextureDesc,
  tex104: TextureDesc,
  tex105: TextureDesc,
  tex106: TextureDesc,
  tex107: TextureDesc,
  tex108: TextureDesc,
  tex109: TextureDesc,
  tex110: TextureDesc,
  tex111: TextureDesc,
  tex112: TextureDesc,
  tex113: TextureDesc,
  tex114: TextureDesc,
  tex115: TextureDesc,
  tex116: TextureDesc,
  tex117: TextureDesc,
  tex118: TextureDesc,
  tex119: TextureDesc,
  tex120: TextureDesc,
  tex121: TextureDesc,
  tex122: TextureDesc,
  tex123: TextureDesc,
  tex124: TextureDesc,
  tex125: TextureDesc,
  tex126: TextureDesc,
  tex127: TextureDesc,
  tex128: TextureDesc,
  tex129: TextureDesc,
  tex130: TextureDesc,
  tex131: TextureDesc,
  tex132: TextureDesc,
  tex133: TextureDesc,
  tex134: TextureDesc,
  tex135: TextureDesc,
  tex136: TextureDesc,
  tex137: TextureDesc,
  tex138: TextureDesc,
  tex139: TextureDesc,
  tex140: TextureDesc,
  tex141: TextureDesc,
  tex142: TextureDesc,
  tex143: TextureDesc,
  tex144: TextureDesc,
  tex145: TextureDesc,
  tex146: TextureDesc,
  tex147: TextureDesc,
  tex148: TextureDesc,
  tex149: TextureDesc,
  tex150: TextureDesc,
  tex151: TextureDesc,
  tex152: TextureDesc,
  tex153: TextureDesc,
  tex154: TextureDesc,
  tex155: TextureDesc,
  tex156: TextureDesc,
  tex157: TextureDesc,
  tex158: TextureDesc,
  tex159: TextureDesc,
  tex160: TextureDesc,
  tex161: TextureDesc,
  tex162: TextureDesc,
  tex163: TextureDesc,
  tex164: TextureDesc,
  tex165: TextureDesc,
  tex166: TextureDesc,
  tex167: TextureDesc,
  tex168: TextureDesc,
  tex169: TextureDesc,
  tex170: TextureDesc,
  tex171: TextureDesc,
  tex172: TextureDesc,
  tex173: TextureDesc,
  tex174: TextureDesc,
  tex175: TextureDesc,
  tex176: TextureDesc,
  tex177: TextureDesc,
  tex178: TextureDesc,
  tex179: TextureDesc,
  tex180: TextureDesc,
  tex181: TextureDesc,
  tex182: TextureDesc,
  tex183: TextureDesc,
  tex184: TextureDesc,
  tex185: TextureDesc,
  tex186: TextureDesc,
  tex187: TextureDesc,
  tex188: TextureDesc,
  tex189: TextureDesc,
  tex190: TextureDesc,
  tex191: TextureDesc,
  tex192: TextureDesc,
  tex193: TextureDesc,
  tex194: TextureDesc,
  tex195: TextureDesc,
  tex196: TextureDesc,
  tex197: TextureDesc,
  tex198: TextureDesc,
  tex199: TextureDesc,
  tex200: TextureDesc,
  tex201: TextureDesc,
  tex202: TextureDesc,
  tex203: TextureDesc,
  tex204: TextureDesc,
  tex205: TextureDesc,
  tex206: TextureDesc,
  tex207: TextureDesc,
  tex208: TextureDesc,
  tex209: TextureDesc,
  tex210: TextureDesc,
  tex211: TextureDesc,
  tex212: TextureDesc,
  tex213: TextureDesc,
  tex214: TextureDesc,
  tex215: TextureDesc,
  tex216: TextureDesc,
  tex217: TextureDesc,
  tex218: TextureDesc,
  tex219: TextureDesc,
  tex220: TextureDesc,
  tex221: TextureDesc,
  tex222: TextureDesc,
  tex223: TextureDesc,
  tex224: TextureDesc,
  tex225: TextureDesc,
  tex226: TextureDesc,
  tex227: TextureDesc,
  tex228: TextureDesc,
  tex229: TextureDesc,
  tex230: TextureDesc,
  tex231: TextureDesc,
  tex232: TextureDesc,
  tex233: TextureDesc,
  tex234: TextureDesc,
  tex235: TextureDesc,
  tex236: TextureDesc,
  tex237: TextureDesc,
  tex238: TextureDesc,
  tex239: TextureDesc,
  tex240: TextureDesc,
  tex241: TextureDesc,
  tex242: TextureDesc,
  tex243: TextureDesc,
  tex244: TextureDesc,
  tex245: TextureDesc,
  tex246: TextureDesc,
  tex247: TextureDesc,
  tex248: TextureDesc,
  tex249: TextureDesc,
  tex250: TextureDesc,
  tex251: TextureDesc,
  tex252: TextureDesc,
  tex253: TextureDesc,
  tex254: TextureDesc,
  tex255: TextureDesc,
  tex256: TextureDesc,
  tex257: TextureDesc,
  tex258: TextureDesc,
  tex259: TextureDesc,
  tex260: TextureDesc,
  tex261: TextureDesc,
  tex262: TextureDesc,
  tex263: TextureDesc,
  tex264: TextureDesc,
  tex265: TextureDesc,
  tex266: TextureDesc,
  tex267: TextureDesc,
  tex268: TextureDesc,
  tex269: TextureDesc,
  tex270: TextureDesc,
  tex271: TextureDesc,
  tex272: TextureDesc,
  tex273: TextureDesc,
  tex274: TextureDesc,
  tex275: TextureDesc,
  tex276: TextureDesc,
  tex277: TextureDesc,
  tex278: TextureDesc,
  tex279: TextureDesc,
  tex280: TextureDesc,
  tex281: TextureDesc,
  tex282: TextureDesc,
  tex283: TextureDesc,
  tex284: TextureDesc,
  tex285: TextureDesc,
  tex286: TextureDesc,
  tex287: TextureDesc,
  tex288: TextureDesc,
  tex289: TextureDesc,
  tex290: TextureDesc,
  tex291: TextureDesc,
  tex292: TextureDesc,
  tex293: TextureDesc,
  tex294: TextureDesc,
  tex295: TextureDesc,
  tex296: TextureDesc,
  tex297: TextureDesc,
  tex298: TextureDesc,
  tex299: TextureDesc,
  tex300: TextureDesc,
  tex301: TextureDesc,
  tex302: TextureDesc,
  tex303: TextureDesc,
  tex304: TextureDesc,
  tex305: TextureDesc,
  tex306: TextureDesc,
  tex307: TextureDesc,
  tex308: TextureDesc,
  tex309: TextureDesc,
  tex310: TextureDesc,
  tex311: TextureDesc,
  tex312: TextureDesc,
  tex313: TextureDesc,
  tex314: TextureDesc,
  tex315: TextureDesc,
  tex316: TextureDesc,
  tex317: TextureDesc,
  tex318: TextureDesc,
  tex319: TextureDesc,
  tex320: TextureDesc,
  tex321: TextureDesc,
  tex322: TextureDesc,
  tex323: TextureDesc,
  tex324: TextureDesc,
  tex325: TextureDesc,
  tex326: TextureDesc,
  tex327: TextureDesc,
  tex328: TextureDesc,
  tex329: TextureDesc,
  tex330: TextureDesc,
  tex331: TextureDesc,
  tex332: TextureDesc,
  tex333: TextureDesc,
  tex334: TextureDesc,
  tex335: TextureDesc,
  tex336: TextureDesc,
  tex337: TextureDesc,
  tex338: TextureDesc,
  tex339: TextureDesc,
  tex340: TextureDesc,
  tex341: TextureDesc,
  tex342: TextureDesc,
  tex343: TextureDesc,
  tex344: TextureDesc,
  tex345: TextureDesc,
  tex346: TextureDesc,
  tex347: TextureDesc,
  tex348: TextureDesc,
  tex349: TextureDesc,
  tex350: TextureDesc,
  tex351: TextureDesc,
  tex352: TextureDesc,
  tex353: TextureDesc,
  tex354: TextureDesc,
  tex355: TextureDesc,
  tex356: TextureDesc,
  tex357: TextureDesc,
  tex358: TextureDesc,
  tex359: TextureDesc,
  tex360: TextureDesc,
  tex361: TextureDesc,
  tex362: TextureDesc,
  tex363: TextureDesc,
  tex364: TextureDesc,
  tex365: TextureDesc,
  tex366: TextureDesc,
  tex367: TextureDesc,
  tex368: TextureDesc,
  tex369: TextureDesc,
  tex370: TextureDesc,
  tex371: TextureDesc,
  tex372: TextureDesc,
  tex373: TextureDesc,
  tex374: TextureDesc,
  tex375: TextureDesc,
  tex376: TextureDesc,
  tex377: TextureDesc,
  tex378: TextureDesc,
  tex379: TextureDesc,
  tex380: TextureDesc,
  tex381: TextureDesc,
  tex382: TextureDesc,
  tex383: TextureDesc,
  tex384: TextureDesc,
  tex385: TextureDesc,
  tex386: TextureDesc,
  tex387: TextureDesc,
  tex388: TextureDesc,
  tex389: TextureDesc,
  tex390: TextureDesc,
  tex391: TextureDesc,
  tex392: TextureDesc,
  tex393: TextureDesc,
  tex394: TextureDesc,
  tex395: TextureDesc,
  tex396: TextureDesc,
  tex397: TextureDesc,
  tex398: TextureDesc,
  tex399: TextureDesc,
  tex400: TextureDesc,
  tex401: TextureDesc,
  tex402: TextureDesc,
  tex403: TextureDesc,
  tex404: TextureDesc,
  tex405: TextureDesc,
  tex406: TextureDesc,
  tex407: TextureDesc,
  tex408: TextureDesc,
  tex409: TextureDesc,
  tex410: TextureDesc,
  tex411: TextureDesc,
  tex412: TextureDesc,
  tex413: TextureDesc,
  tex414: TextureDesc,
  tex415: TextureDesc,
  tex416: TextureDesc,
  tex417: TextureDesc,
  tex418: TextureDesc,
  tex419: TextureDesc,
  tex420: TextureDesc,
  tex421: TextureDesc,
  tex422: TextureDesc,
  tex423: TextureDesc,
  tex424: TextureDesc,
  tex425: TextureDesc,
  tex426: TextureDesc,
  tex427: TextureDesc,
  tex428: TextureDesc,
  tex429: TextureDesc,
  tex430: TextureDesc,
  tex431: TextureDesc,
  tex432: TextureDesc,
  tex433: TextureDesc,
  tex434: TextureDesc,
  tex435: TextureDesc,
  tex436: TextureDesc,
  tex437: TextureDesc,
  tex438: TextureDesc,
  tex439: TextureDesc,
  tex440: TextureDesc,
  tex441: TextureDesc,
  tex442: TextureDesc,
  tex443: TextureDesc,
  tex444: TextureDesc,
  tex445: TextureDesc,
  tex446: TextureDesc,
  tex447: TextureDesc,
  tex448: TextureDesc,
  tex449: TextureDesc,
  tex450: TextureDesc,
  tex451: TextureDesc,
  tex452: TextureDesc,
  tex453: TextureDesc,
  tex454: TextureDesc,
  tex455: TextureDesc,
  tex456: TextureDesc,
  tex457: TextureDesc,
  tex458: TextureDesc,
  tex459: TextureDesc,
  tex460: TextureDesc,
  tex461: TextureDesc,
  tex462: TextureDesc,
  tex463: TextureDesc,
  tex464: TextureDesc,
  tex465: TextureDesc,
  tex466: TextureDesc,
  tex467: TextureDesc,
  tex468: TextureDesc,
  tex469: TextureDesc,
  tex470: TextureDesc,
  tex471: TextureDesc,
  tex472: TextureDesc,
  tex473: TextureDesc,
  tex474: TextureDesc,
  tex475: TextureDesc,
  tex476: TextureDesc,
  tex477: TextureDesc,
  tex478: TextureDesc,
  tex479: TextureDesc,
  tex480: TextureDesc,
  tex481: TextureDesc,
  tex482: TextureDesc,
  tex483: TextureDesc,
  tex484: TextureDesc,
  tex485: TextureDesc,
  tex486: TextureDesc,
  tex487: TextureDesc,
  tex488: TextureDesc,
  tex489: TextureDesc,
  tex490: TextureDesc,
  tex491: TextureDesc,
  tex492: TextureDesc,
  tex493: TextureDesc,
  tex494: TextureDesc,
  tex495: TextureDesc,
  tex496: TextureDesc,
  tex497: TextureDesc,
  tex498: TextureDesc,
  tex499: TextureDesc,
  tex500: TextureDesc

}
pub fn main(){
  
}
